use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, CustomizeConnection};
use diesel::pg::PgConnection;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use crate::schema::posts;
use crate::schema::posts::dsl;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn env_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
}

/// テスト時にCommitしないtransactionを提供するtrait.
///
/// # example
///
/// ```
/// use actix_web::{Error, http, web};
/// use diesel::r2d2::{self, ConnectionManager};
/// use diesel::pg::PgConnection;
/// use my_app::db::posts::{env_database_url, TestTransaction, DbPool};
///
/// let manager = ConnectionManager::<PgConnection>::new(env_database_url());
/// let pool: DbPool = r2d2::Pool::builder()
///     .connection_customizer(Box::new(TestTransaction))
///     .build(manager)
///     .expect("Failed to init pool");
/// ```
#[derive(Debug)]
pub struct TestTransaction;

impl CustomizeConnection<PgConnection, r2d2::Error> for TestTransaction {
    fn on_acquire(
        &self,
        conn: &mut PgConnection,
    ) -> ::std::result::Result<(), r2d2::Error> {
        conn.begin_test_transaction().unwrap();
        Ok(())
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn create_post<'a>(connection: &PgConnection, post: NewPost) -> Result<Post, diesel::result::Error> {
    diesel::insert_into(posts::table)
        .values(post)
        .get_result::<Post>(connection)
}

pub fn show_post<'a>(connection: &PgConnection, count: i64, page: i64) -> Result<Vec<Post>, diesel::result::Error> {
    let offset = count * (page - 1);

    dsl::posts.filter(dsl::published.eq(true))
        .limit(count)
        .offset(offset)
        .order(dsl::id.desc())
        .load::<Post>(connection)
}

pub fn publish_post<'a>(connection: &PgConnection, target_id: i32) -> Result<Post, diesel::result::Error> {
    diesel::update(dsl::posts.find(target_id))
        .set(dsl::published.eq(true))
        .get_result::<Post>(connection)
}

#[cfg(test)]
mod test {
    use super::*;

    fn init() -> PgConnection {
        let database_url = env_database_url();
        let db = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        db.begin_test_transaction().unwrap();
        db
    }

    #[test]
    fn scenario() {
        let connection = init();

        let new_post1 = NewPost {
            title: "unit test title111",
            body: "unit test body111",
        };

        let created_posts = create_post(&connection, new_post1).unwrap();
        let _published_post = publish_post(&connection, created_posts.id);

        let new_post2 = NewPost {
            title: "unit test title222",
            body: "unit test body222",
        };

        let created_posts = create_post(&connection, new_post2).unwrap();
        let _published_post = publish_post(&connection, created_posts.id);

        let posts = show_post(&connection, 2, 1).unwrap();

        let result = posts
            .iter()
            .map(|item| {
                item.title.clone()
            })
            .collect::<Vec<String>>();

        assert_eq!(result, ["unit test title222", "unit test title111"]);
    }
}