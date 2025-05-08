# mini grep

一个简单的可以从文件中搜索字符串的工具。

首先运行
```shell
cargo build
```
然后在`\target\debug`目录下找到`minigrep.exe`

使用方法
```shell
minigrep.exe [text] [file.txt]
```
就可以在`file.txt`中搜索`text`。

如果需要忽略大小，可以设置环境变量。在powershell下方法如下

```shell
$Env:IGNORE_CASE = 1;
```


## 读取参数
首先我们需要让 grep 可以接受两个参数，分别是文件路径和要搜索的字符串。

我们首先需要用到一个Rust标准库提供的函数`std::env::args`。

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```
这样可以获取到参数，并使用`collect()`创建了一个包含迭代器所有值的vector。然后就是用宏打印出了接受的参数。
```
cargo run -- needle haystack
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running `target\debug\minigrep.exe needle haystack`
[src\main.rs:5:5] args = [
    "target\\debug\\minigrep.exe",
    "needle",
    "haystack",
]
```
可以看到有三个参数，分别是可以执行文件以及两个参数。返回可执行文件是为了对应C语言的接口。
> args 不能接受无效的Unicode,如果是无效的Unicode就会触发panic。


## 将参数保存进变量

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
```

## 读取文件
这里需要使用`std::fs`来处理文件。
```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    
    let contents = fs::read_to_string(file_path);
    println!("In file {}", file_path);
    println!("contents is :\n{}",contents.unwrap());
}
```
这里是从`file_path`文件读取字符串到`contents`。

打印的时候为什么需要用`unwrap`?这是因为`read_to_string`函数的返回值是`Ruslt<String>`。

这里在项目的根目录下创建文件`poem.txt`，这样在运行是，把`poem.txt`作为第二个参数，程序就可以读取并输出文件的内容。

## 拆分main函数
以下代码就可以把一些较为独立的函数从main函数中拆分出来
```rust
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
```

## 拆分文件
新建`src/lib.rs`文件，并且把`main.rs`进行拆分。

`main.rs`

```rust
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
```

`lib.rs`

```rust
use std::fs;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub file_path: String,
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    println!("contents is :\n{}", contents);
    return Ok(());
}
```
这样就可以对这个库写一些测试。

## 编写测试
我们可以在`lib.rs`添加以下内容作为测试的例子。
```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return vec![];
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
```
这个测试可以正常运行的。但是会报错。因为这里并没有真的实现`search`函数。
## 实现search函数

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    return res;
}
```
这样就实现了search函数，并且可以通过刚才的测试了。

## 支持大小写不敏感
首先将main函数继续拆分，使得main函数中完全没有实现，只有调用。
`main.rs`
```rust
use minigrep;
use std::process;

fn main() {
    let config = minigrep::read().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("{}", err);
        process::exit(1);
    };
}
```

然后对`Config`新增一个变量`ignore_case`。这个值根据环境变量来决定。

新增一个函数`search_case_insensitive`，原理是在匹配时将文本串与目标串都变成纯小写。

```rust
use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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
            ignore_case: env::var("IGNORE_CASE").is_ok() && env::var("IGNORE_CASE").unwrap() == "1",
        });
    }
}

pub fn read() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).map_err(|err| format!("参数错误 : {}", err))?;
    return Ok(config);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents =
        fs::read_to_string(config.file_path).map_err(|err| format!("运行错误 : {}", err))?;

    println!("ignore-case {}", config.ignore_case);
    for line in contents.lines() {
        println!("{}", line);
    }
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
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        }
    }
    return res;
}
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
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
```

## 迭代器优化性能
看`Config::build`

```rust
fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        } else if args.len() > 3 {
            return Err("参数过多");
        }
        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok() && env::var("IGNORE_CASE").unwrap() == "1",
        });
    }
```
为了返回一个有所有权的`Result`,这里实现两个`clone`深拷贝。
我可以通过获得迭代的所有权，这样就不用触发深拷贝了。
更新以下两个函数即可使用迭代器。
```rust
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


pub fn read() -> Result<Config, String> {
    let config = Config::build(env::args()).map_err(|err| format!("参数错误 : {}", err))?;
    return Ok(config);
}
```

另一个可以用迭代优化的函数就是搜索函数。
```rust
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
```