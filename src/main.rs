use std::{env, process};

use subper::{Config, run};
fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing argument: {}", err);
        process::exit(1);
    });
    run(config);
}

