use crate::{repository::task_repository::TaskRepository, service::task_service::TaskService};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::envs::Config;
use dotenv::dotenv;
use model::model_manager::ModelManager;
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod config;
mod errors;
mod middleware;
mod model;
mod repository;
mod router;
mod routes;
mod service;
mod util;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secrets: SecretStore) -> shuttle_axum::ShuttleAxum {
    dotenv().ok();
    let config = Config::init(secrets.clone());

    let mm = ModelManager::new(&config).await.unwrap();

    let task_repository = Arc::new(TaskRepository::new(mm.clone()));

    let task_service = Arc::new(TaskService::new(task_repository));
    let client_url = secrets.get("CLIENT_URL").expect("CLIENT_URL must be set");

    let cors = CorsLayer::new()
        .allow_origin(client_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let routes = routes::new(cors, config, task_service.clone());
    Ok(routes.await.into())
}
