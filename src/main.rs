use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("参数错误: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("运行错误: {}", e);
        process::exit(1);
    };
}