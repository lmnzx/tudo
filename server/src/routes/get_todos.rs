use poem::{handler, IntoResponse};

#[handler]
pub fn get_todos() -> impl IntoResponse {
    "Get todos".into_response()
}
