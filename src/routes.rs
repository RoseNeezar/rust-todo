use std::sync::Arc;

use axum::{
    http::{self, header::AUTHORIZATION, request::Parts, HeaderValue},
    routing::get,
    Router,
};
use rspc::{Config, Error, ErrorCode};
use rspc_axum;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use uuid::Uuid;
pub type PublicRouter = rspc::Router<Api>;
pub type PrivateRouter = rspc::Router<UserCtx>;

use crate::{
    config, errors::ErrorResponse, middleware::get_user, router::todo_router::todo_router,
    service::task_service::TaskService,
};

fn api_router() -> PublicRouter {
    PublicRouter::new()
        .config(
            Config::new().export_ts_bindings(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join(".")
                    .join("web//src/utils/api.ts"),
            ),
        )
        .middleware(|mw| {
            mw.middleware(|mw| async move {
                let tmp: Api = mw.ctx.clone();

                let user_id = match get_user(tmp.token, tmp.config.jwt_secret).await {
                    Some(data) => data,
                    None => {
                        return Err(Error::new(
                            ErrorCode::InternalServerError,
                            ErrorResponse::InvalidRequest {
                                error: "unauthorized user".to_string(),
                            }
                            .to_string(),
                        ))
                    }
                };

                Ok(mw.with_ctx(UserCtx {
                    task_service: tmp.task_service,
                    user_id: Some(user_id),
                }))
            })
        })
        .merge("todo.", todo_router())
        .build()
}

#[derive(Clone, Debug)]
pub struct Api {
    pub task_service: Arc<TaskService>,
    pub token: Option<HeaderValue>,
    pub config: config::envs::Config,
}

#[derive(Clone, Debug)]
pub struct UserCtx {
    pub task_service: Arc<TaskService>,
    pub user_id: Option<Uuid>,
}
use http::HeaderMap;
use std::ops::Deref;

pub struct UnauthedCtx(pub HeaderMap);
impl Deref for UnauthedCtx {
    type Target = HeaderMap;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn new(
    cors: CorsLayer,
    config: config::envs::Config,
    task_service: Arc<TaskService>,
) -> Router {
    let rspc = api_router().arced();

    axum::Router::new()
        .route("/", get(|| async { "Hello 'rspc'!" }))
        .nest(
            "/rspc",
            rspc_axum::endpoint(rspc, |parts: Parts| {
                let token = parts.headers.get(AUTHORIZATION).cloned();
                Api {
                    task_service,
                    token,
                    config,
                }
            }),
        )
        .layer(cors)
}
