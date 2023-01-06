use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::build(&args) {
        Ok(conf) => conf,
        Err(msg) => { println!("{}", msg); process::exit(1); }
    };

    println!("Looking for regexp: {}", config.query);
    println!("Looking inside {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Exiting with error: {}", e);
        process::exit(1);
    }
}


