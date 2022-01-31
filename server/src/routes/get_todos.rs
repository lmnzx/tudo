use futures::stream::TryStreamExt;
use mongodb::{bson::Document, Collection};
use poem::{
    handler,
    web::{Data, Json},
    IntoResponse,
};

#[handler]
pub async fn get_todos(collection: Data<&Collection<Document>>) -> impl IntoResponse {
    let result = collection
        .find(None, None)
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    println!("{:?}", result);

    Json(serde_json::to_value(result).unwrap())
}
