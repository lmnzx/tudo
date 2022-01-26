use poem::{handler, http::StatusCode, IntoResponse};

#[handler]
pub fn ping() -> impl IntoResponse {
    StatusCode::OK
}
