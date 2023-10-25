use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching {} in file '{}'", config.query, config.filename);

    if let Err(err) = minigrep::run(&config) {
        eprintln!("application error: {}", err);
        process::exit(1);
    };
}
