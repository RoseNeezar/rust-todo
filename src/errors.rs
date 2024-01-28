use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorResponse {
    #[error("invalid_request")]
    InvalidRequest,
    #[error("no_todo")]
    NoTodo,
}
