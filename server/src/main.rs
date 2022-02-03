use dotenv::dotenv;
use mongodb::{bson::Document, Client};
use poem::{
    get,
    listener::TcpListener,
    middleware::{AddData, Tracing},
    post, EndpointExt, Route, Server,
};
use std::env;

use server::routes::{create_todo, get_todos, ping};

/*
    TODO - CRUD operations on todo
    TODO - Add database
    TODO - Add authentication
*/

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let db = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", db);

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug")
    }

    tracing_subscriber::fmt::init();

    let mongo_client = Client::with_uri_str(&db)
        .await
        .unwrap()
        .database("rusty-todos");

    let todo_collection = mongo_client.collection::<Document>("todos");

    let app = Route::new()
        .at("/ping", get(ping))
        .at("/create_todo", post(create_todo))
        .at("/get_todos/*", get(get_todos))
        .with(AddData::new(todo_collection))
        .with(Tracing::default());

    let address = TcpListener::bind("127.0.0.1:3000");

    Server::new(address)
        .run_with_graceful_shutdown(
            app,
            async move {
                let _ = tokio::signal::ctrl_c().await;
            },
            Some(tokio::time::Duration::from_secs(5)),
        )
        .await
}
