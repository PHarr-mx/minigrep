# mini grep


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

# 拆分文件
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

# 编写测试
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
# 实现search函数

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