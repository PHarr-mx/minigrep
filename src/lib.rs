use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        } else if args.len() > 3 {
            return Err("参数过多");
        }
        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}

pub fn read() -> Result<Config, String>{
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).map_err(|err| format!("参数错误 : {}", err))?;
    return Ok(config);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).map_err(|err| format!("运行错误 : {}", err))?;
    let search_results = search(&config.query, &contents);
    if search_results.is_empty(){
        println!("未找到");
    } else {
        println!("搜索结果 : ");
        for line in search(&config.query, &contents) {
            println!("{}",line);
        }
        println!("共计出现 : {} 次",  search_results.len());
    }
    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "万";
        let contents = "北国风光，千里冰封，万里雪飘。\n望长城内外，惟余莽莽；大河上下，顿失滔滔。";
        assert_eq!(vec!["北国风光，千里冰封，万里雪飘。"], search(query, contents));
    }
}