use std::sync::Arc;

use axum::{routing::get, Router};
use rspc::Config;
pub use rspc::RouterBuilder;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;

pub type PublicRouter = rspc::Router<Api>;

use crate::{router::todo_router::todo_router, service::task_service::TaskService};

fn api_router() -> rspc::Router<Api> {
    PublicRouter::new()
        .config(
            Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(".")
                    .join("web//src/utils/api.ts"),
            ),
        )
        .merge("todo.", todo_router())
        .build()
}

#[derive(Clone, Debug)]
pub struct Api {
    pub task_service: Arc<TaskService>,
}

pub fn new(cors: CorsLayer, task_service: Arc<TaskService>) -> axum::Router {
    let rspc = api_router().arced();

    Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest("/rspc", rspc.endpoint(|| Api { task_service }).axum())
        .layer(cors)
}
