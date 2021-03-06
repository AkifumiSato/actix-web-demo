use crate::driver::pool::DbPool;
use crate::driver::posts::PostTable;
use crate::usecase::article_find::{self, InputData};
use actix_web::{web, HttpResponse};

pub async fn index(pool: web::Data<DbPool>, info: web::Path<InputData>) -> HttpResponse {
    let connection = pool
        .get()
        .expect("couldn't get driver connection from pool");
    let post_table = PostTable::new(&connection);

    match article_find::execute(post_table, info.into_inner()) {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
