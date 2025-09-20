use std::env;

use actix_web::middleware::from_fn;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, middleware, web};
use dotenv::dotenv;
use env_logger::{Env, init_from_env};
use mockservice::api::equifax_block::router::get_equifax_block_info;
use mockservice::api::equifax_converter::router::get_equifax_converter_info;
use mockservice::api::equifax_fps::router::get_equifax_fps_info;
use mockservice::api::equifax_payment::router::get_equifax_payment_info;
use mockservice::api::equifax_scoring::router::get_equifax_scoring_info;
use mockservice::api::equifax_v4::router::get_equifax_v4_info;
use mockservice::api::mock_response::add_mock_response;
use mockservice::api::vk::router::get_vk_info;
use mockservice::database::{check_connection, get_connection_pool};
use mockservice::middleware::auth::extract_auth_key;
use serde_json::json;

#[get("/ping")]
async fn check_ping() -> impl Responder {
    HttpResponse::Ok().json(json!({"success": true}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    init_from_env(Env::default().default_filter_or("info"));
    check_connection().await.unwrap();
    let max_connections: u32 = match env::var("MAX_CONNECTIONS") {
        Ok(value) => match value.parse::<u32>() {
            Ok(value) => value,
            Err(_) => panic!("MAX_CONNECTION must be positive integer"),
        },
        Err(_) => 4,
    };
    let current_workers: u32 = match env::var("WORKERS") {
        Ok(value) => match value.parse::<u32>() {
            Ok(val) => val,
            Err(_) => panic!("WORKERS must be positive integer"),
        },
        Err(_) => 1,
    };
    let pool = get_connection_pool(max_connections).await.unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(from_fn(extract_auth_key))
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(check_ping)
            .service(
                web::scope("/api")
                    .service(get_vk_info)
                    .service(add_mock_response)
                    .service(get_equifax_v4_info)
                    .service(get_equifax_block_info)
                    .service(get_equifax_converter_info)
                    .service(get_equifax_fps_info)
                    .service(get_equifax_payment_info)
                    .service(get_equifax_scoring_info),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .workers(current_workers as usize)
    .run()
    .await
}
