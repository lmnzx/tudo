use poem::{get, listener::TcpListener, middleware::Tracing, EndpointExt, Route, Server};

use server::routes::{create_todo, ping};

//* New Impelementation of rusty-todos, with client server architecture

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug")
    }

    tracing_subscriber::fmt::init();

    let app = Route::new()
        .at("/ping", get(ping))
        .at("/create_todo", get(create_todo))
        .with(Tracing::default());

    let address = TcpListener::bind("127.0.0.1:3000");

    Server::new(address).run(app).await
}
