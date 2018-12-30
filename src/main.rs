use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = lego::Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = lego::run(config) {
        eprintln!("Application error: {}", err);

        process::exit(1);
    }
}

