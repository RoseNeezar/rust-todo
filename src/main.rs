use crate::{repository::task_repository::TaskRepository, service::task_service::TaskService};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::config::Config;
use dotenv::dotenv;
use model::model_manager::ModelManager;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod model;
mod repository;
mod routes;
mod service;
mod util;
mod views;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::init();

    let mm = ModelManager::new(&config).await.unwrap();

    let task_repository = Arc::new(TaskRepository::new(mm.clone()));

    let task_service = Arc::new(TaskService::new(task_repository));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let routes = routes::new(cors, task_service.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
