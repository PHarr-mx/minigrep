use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("参数错误: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("运行错误: {}", e);
        process::exit(1);
    };
}

struct Config{
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("参数不够");
        } else if args.len() > 3 {
            return Err("参数过多");
        }
        return Ok(Config{
            query: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("contents is :\n{}", contents);
    return Ok(());
}