use std::str::FromStr;

use anyhow::{anyhow, Result};
use reqwest::Url;
use structopt::StructOpt;

// get 子命令

/// feed get with an url and we will retrieve the response for you 
#[derive(Debug, StructOpt)]
pub struct Get {
    /// HTTP请求的URL
    #[structopt(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    // 检查URL是否合法
    let _url : Url = s.parse()?;

    Ok(s.into())
}

// post子命令。需要输入一个URL， 和若干可选的key=value, 用于提供json body

/// feed post with an url and optional key=value pairs. We will post 
/// as JSON , and retrieve the response for you
#[derive(Debug, StructOpt)]
pub struct Post {
    /// HTTP 请求的URL
    #[structopt(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的body
    #[structopt(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的key=value 可以通过parse_kv_pair解析成KvPair结构
#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}


impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行split, 这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));

        Ok(Self {
            // 从迭代器中取第一个结果作为key，迭代器返回Some(T)/None
            // 我们将其转换成Ok(T)/Err(E), 然后用?处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为为KyPair实现了FromStr,这里可以直接使用s.parse()得到KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

// 子命令分别对应不同的HTTP方法，目前只支持get / post
#[derive(Debug, StructOpt)]
pub enum SubCommand {
    #[structopt(name = "get")]
    Get(Get),
    #[structopt(name = "post")]
    Post(Post),
    // 其他方法
}

// 定义HTTPie的CLI的主入口，它包含若干子命令
// 下面/// 的注释是文档，structopt会将其作为CLI的帮助

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Debug, StructOpt)]
#[structopt(name = "httpie", author = "davirain.yin@gmail.com")]
pub struct App {
    #[structopt(subcommand)]
    pub subcommand: SubCommand,
}

fn main() {
    let opt = App::from_args();
    println!("{:?}", opt);
}