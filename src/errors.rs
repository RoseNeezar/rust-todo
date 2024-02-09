use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorResponse {
    #[error("invalid_request, error={error}")]
    InvalidRequest { error: String },
    #[error("no_todo with id={id}")]
    NoTodo { id: String },
}
