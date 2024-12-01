use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Reading file: {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
        .expect("Can't read the file");

    println!("Read:\n{content}");
}

struct Config {
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Need filename as first argument.");
        }

        let file_path = args[1].clone();

        Config { file_path }
    }
}
