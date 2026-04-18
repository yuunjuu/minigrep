use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("{}에서 {}을 검색합니다.", file_path, query);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("\n내용: \n{contents}");
}