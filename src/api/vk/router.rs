use actix_web::{HttpResponse, Responder, post, web};
use sqlx::{Pool, Postgres};

use crate::api::{get_file_path, mock_fetch_file_response, schemas::PublicInfoRequest};

const SERVICE_ID: i32 = 8;

#[post("/vk")]
async fn get_vk_info(
    pool: web::Data<Pool<Postgres>>,
    input_info: web::Json<PublicInfoRequest>,
) -> actix_web::Result<impl Responder> {
    let path = get_file_path("vk.json")?;
    match mock_fetch_file_response(pool.get_ref(), input_info.into_inner(), SERVICE_ID, path).await
    {
        Ok(val) => Ok(HttpResponse::Ok()
            .content_type("text/plain; charset=utf-8")
            .body(val)),
        Err(resp) => Ok(resp),
    }
}
