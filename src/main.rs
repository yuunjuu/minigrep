use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("인수를 파싱하는 중 에러가 발생했습니다. {err}");
        process::exit(1);
    });
    println!("{}에서 {}을 검색합니다.", config.file_path, config.query);
    if let Err(e) = run(config) {
        println!("실행 중 에러가 발생했습니다. {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("검색을 위한 인수가 충분하지 않습니다.");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("\n내용: \n{contents}");
    Ok(())
}