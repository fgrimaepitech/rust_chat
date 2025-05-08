use actix_web::{App, HttpServer, web, middleware};
use actix_cors::Cors;
mod handlers;
mod crypto;
use handlers::{post_message, get_messages, create_channel, list_channels, join_channel, AppState};
use crypto::Encryption;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());
    let redis_client = redis::Client::open(redis_url).expect("Invalid REDIS_URL");
    let encryption = Encryption::new().expect("Failed to initialize encryption");
    let state = AppState { 
        redis_client,
        encryption,
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(post_message)
            .service(get_messages)
            .service(create_channel)
            .service(list_channels)
            .service(join_channel)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
