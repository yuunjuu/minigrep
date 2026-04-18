use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("{}에서 {}을 검색합니다.", config.file_path, config.query);
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    println!("\n내용: \n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
