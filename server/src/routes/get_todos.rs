use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, Document},
    Collection,
};
use poem::{
    handler,
    http::StatusCode,
    web::{Data, Json},
    IntoResponse, Request,
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Params {
    id: Option<String>,
    title: Option<String>,
}

#[handler]
pub async fn get_todos(
    collection: Data<&Collection<Document>>,
    req: &Request,
) -> impl IntoResponse {
    let result = collection
        .find(None, None)
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let params = req.params::<Params>().unwrap();

    // TODO - Add filtering

    println!("{:?}", result);

    Json(serde_json::to_value(result).unwrap())
}
