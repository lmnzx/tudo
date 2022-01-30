use mongodb::{
    bson::{doc, Document},
    Collection,
};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

use crate::todo::Todo;

#[handler]
pub async fn create_todo(
    Json(todo): Json<Todo>,
    collection: Data<&Collection<Document>>,
) -> impl IntoResponse {
    println!("{:?}", todo);

    let result = collection
        .insert_one(
            doc! {
                "title": todo.title,
                "completed": todo.completed,
            },
            None,
        )
        .await
        .unwrap();

    println!("{:?}", result);

    "Created a new todo".into_response();
}
