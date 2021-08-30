use anyhow::Result;
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
    url: String,
    /// HTTP 请求的body
    body: Vec<String>,
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