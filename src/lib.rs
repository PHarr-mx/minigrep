use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("未找到查询字符串"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("未找到文件路径"),
        };

        if args.count() > 0 {
            return Err("参数过多");
        }

        return Ok(Config {
            query,
            file_path,
            ignore_case: env::var("IGNORE_CASE").is_ok() && env::var("IGNORE_CASE").unwrap() == "1",
        });
    }
}

pub fn read() -> Result<Config, String> {
    let config = Config::build(env::args()).map_err(|err| format!("参数错误 : {}", err))?;
    return Ok(config);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).map_err(|err| format!("运行错误 : {}", err))?;

    let search_results = match config.ignore_case {
        true => search_case_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };
    if search_results.is_empty() {
        println!("未找到");
    } else {
        println!("搜索结果 : ");
        for line in &search_results {
            println!("{}", line);
        }
        println!("共计出现 : {} 次", search_results.len());
    }
    return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "万";
        let contents = "北国风光，千里冰封，万里雪飘。\n望长城内外，惟余莽莽；大河上下，顿失滔滔。";
        assert_eq!(
            vec!["北国风光，千里冰封，万里雪飘。"],
            search(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "RusT";
        let contents = "Rust\nrUst\nrus";
        assert_eq!(
            search_case_insensitive(query, contents),
            vec!["<Rust", "rUst"]
        );
    }
}
