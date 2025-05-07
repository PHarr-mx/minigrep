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
