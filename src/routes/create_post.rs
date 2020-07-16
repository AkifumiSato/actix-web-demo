use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::db::{NewPost, create_post, DbPool};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    title: String,
    body: String,
}

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<PostJson>
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get db connection from pool");

    let new_post = NewPost {
        title: &item.title,
        body: &item.body,
    };

    match create_post(&connection, new_post) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}