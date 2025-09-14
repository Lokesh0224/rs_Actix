mod api; 
mod repository;

use api::task::{
    get_task
}

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).app_data().service(get_task)
    })
    .bind(("127.0.0.1", 80))?
    .run()
    .await
}
