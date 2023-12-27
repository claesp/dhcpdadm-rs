mod admin;
mod api;

use actix_web::{middleware::Logger, App, HttpServer};
use admin::agent::get_agent;
use api::agent::get_agents;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(get_agent)
            .service(get_agents)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
