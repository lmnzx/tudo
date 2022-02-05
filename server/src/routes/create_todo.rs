use mongodb::{
    bson::{doc, Document},
    Collection,
};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

#[handler]
pub async fn create_todo(
    Json(todo): Json<Todo>,
    collection: Data<&Collection<Document>>,
) -> impl IntoResponse {
    println!("{:?}", todo);

    match collection
        .insert_one(
            doc! {
                "title": todo.title,
                "completed": todo.completed,
            },
            None,
        )
        .await
    {
        Ok(_) => "Todo created".into_response(),
        Err(e) => format!("Error: {}", e).into_response(),
    }
}
