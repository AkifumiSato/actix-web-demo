use actix_web::{web, HttpResponse};
use crate::driver::pool::DbPool;
use crate::usecase::post_delete::{self, InputData};

pub async fn index(
    pool: web::Data<DbPool>,
    item: web::Json<InputData>,
) -> HttpResponse {
    let connection = pool.get().expect("couldn't get driver connection from pool");
    let input = item.into_inner();
    let id = input.id;

    match post_delete::execute(&connection, input) {
        Ok(_v) => HttpResponse::Ok().body(format!("delete post [{}]", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}