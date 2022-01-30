use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(
    author = "John Doe",
    version = "1.0.0",
    about = "A simple program to greet a person"
)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
