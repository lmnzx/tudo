use poem::{handler, IntoResponse};

#[handler]
pub fn create_todo() -> impl IntoResponse {
    "New todo created".into_response()
}
