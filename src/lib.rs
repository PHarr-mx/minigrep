use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("contents is :\n{}", contents);
    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
