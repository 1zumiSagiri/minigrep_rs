use std::env;
use std::process;

use minigrep_rs::Config;

fn main() {
    // let args : Vec<String> = env::args().collect();
    // if cfg!(debug_assertions) {
    //     dbg!(&args);
    // }

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep_rs::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
