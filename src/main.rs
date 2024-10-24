mod db;
mod fetcher;
mod handlers;
mod models;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use db::config::DbConfig;
use db::crud_trait::HistoryCRUD;
use db::mongo_db::MongoDbStore;
use dotenv::dotenv;
use env_logger;
use handlers::hist_handlers::{
    add_depth_history, add_earnings_history, add_rune_pool_history, add_swap_history,
    get_depth_history, get_earnings_history, get_runepool_history, get_swap_history,
    update_depth_history, update_earnings_history, update_runepool_history, update_swap_history,
};
use std::env;
use std::sync::Arc;

#[get("/test")]
async fn checker() -> impl Responder {
    HttpResponse::Ok().body("Hello Medium!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let config = DbConfig::new(
        "mongodb",
        env::var("DATABASE_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017/?directConnection=true".to_string()),
        "rustest",
    );

    let repo = MongoDbStore::new(&config).await;

    let store_arc: Arc<dyn HistoryCRUD> = Arc::new(repo);
    let store_data: Data<dyn HistoryCRUD> = Data::from(store_arc);

    HttpServer::new(move || {
        App::new()
            .app_data(store_data.clone())
            .service(get_depth_history)
            .service(get_earnings_history)
            .service(get_swap_history)
            .service(get_runepool_history)
            .service(add_depth_history)
            .service(add_earnings_history)
            .service(add_swap_history)
            .service(add_rune_pool_history)
            .service(update_depth_history)
            .service(update_swap_history)
            .service(update_earnings_history)
            .service(update_runepool_history)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
