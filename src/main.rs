use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1).expect("Query was not provided")
    let file_path = args.get(2).expect("File path was not provided");

    println!("Looking inside {}", file_path.to_string());

}
