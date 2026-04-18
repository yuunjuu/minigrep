use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    println!("{}에서 {}을 검색합니다.", config.file_path, config.query);
    if let Err(e) = minigrep::run(config) {
        eprintln!("{e}");
        process::exit(1);
    }
}
