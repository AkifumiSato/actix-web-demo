use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::db::{establish_connection, show_post, Post};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostJson {
    page: Option<i64>,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    result: Vec<Post>,
}

pub async fn index(item: web::Json<PostJson>) -> HttpResponse {
    let connection = establish_connection();

    let page = match item.page {
        Some(x) => x,
        None => 1,
    };

    match show_post(&connection, item.count, page) {
        Ok(posts) => HttpResponse::Ok().json(Response {
            result: posts
        }),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, Error, web, App};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let mut app = test::init_service(
            App::new().route("/", web::post().to(index)),
        )
            .await;

        // status test
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&PostJson {
                page: None,
                count: 5,
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        // response test
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&PostJson {
                page: None,
                count: 5,
            })
            .to_request();
        let resp: Response = test::read_response_json(&mut app, req).await;
        resp.result.iter().for_each(|post| {
            assert_eq!(post.id, 1);
        });

        Ok(())
    }
}