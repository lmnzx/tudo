use clap::App;

fn ping() -> String {
    match ureq::get("http://localhost:3000/ping").call() {
        Ok(res) => {
            println!("{}", res.into_string().unwrap());
            "Pong".to_string()
        }
        Err(err) => {
            format!("Error - {}", err)
        }
    }
}

fn main() {
    let matches = App::new("rusty-todo")
        .version("0.1.0")
        .author("Sayan Mallick <snyxmk@gmail.com>")
        .about("Rusty Todo CLI")
        .subcommand(App::new("ping").about("sends a ping to the server"))
        .get_matches();

    match matches.subcommand() {
        Some(("ping", _)) => {
            println!("{}", ping());
        }
        _ => println!("Yet to implement"),
    }
}
