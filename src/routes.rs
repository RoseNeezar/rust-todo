use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use rspc::Config;
pub use rspc::RouterBuilder;
use serde::Deserialize;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct Ctx {}
pub type PublicRouter = rspc::Router<Ctx>;

use crate::service::task_service::TaskService;
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub(crate) fn what() -> RouterBuilder<Ctx> {
    PublicRouter::new()
        .config(
            Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(".")
                    .join("web//src/utils/api.ts"),
            ),
        )
        .query("omega=what", |t| t(|_, _: ()| Ok("ok")))
}

#[derive(Clone)]
pub struct Api {
    task_service: Arc<TaskService>,
}

#[derive(Debug, Deserialize)]
struct GetTodoPayload {
    id: i64,
}

async fn api_get_todo_handler(
    State(mm): State<Api>,
    Json(payload): Json<GetTodoPayload>,
) -> impl IntoResponse {
    let todo = mm.task_service.get_todo(payload.id).await;
    dbg!("{:?}", &todo);

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(e) => match e {
            _ => Err(e.to_string()),
        },
    }
}

pub fn new(cors: CorsLayer, task_service: Arc<TaskService>) -> axum::Router {
    let api = Api { task_service };

    let rspc = what().build().arced();

    Router::new()
        .route("/api/get", get(api_get_todo_handler))
        .nest("/rspc", rspc.endpoint(|| Ctx {}).axum())
        .with_state(api.clone())
        .with_state(api)
        .layer(cors)
}
