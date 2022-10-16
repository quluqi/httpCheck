use reqwest::cookie::Jar;
use serde::Deserialize;
use std::{env::args, fs, sync::Arc, time::Duration};
use tokio::join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //build args
    let args: Vec<String> = std::env::args().collect();
    let mut filename = String::from("./config/urls.json");
    if args.len() == 2 {
        filename = args.get(1).unwrap().clone();
    }
    let config = fs::read_to_string(filename).unwrap();
    let urlconfig: UrlConfig = serde_json::from_str(config.as_str()).unwrap();

    //build async http
    let mut handles = Vec::with_capacity(urlconfig.urls.len());
    for i in urlconfig.urls {
        handles.push(tokio::spawn(req_http(i)));
    }

    //await result
    for handle in handles {
        handle.await?;
    }
    Ok(())
}

async fn req_http(url: String) -> String {
    let url: reqwest::Url = url.parse().unwrap();
    println!("req_http:{}", url);

    let mut headers = std::collections::HashMap::new();
    headers.insert("demo", "health-check");
    let cookie = "health-cookie";
    let jar = Jar::default();
    jar.add_cookie_str(cookie, &url);

    let client = reqwest::Client::builder()
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();

    let res = client.get(url).send().await;
    match res {
        Ok(response) => println!("status:{},{}", response.status(), response.url()),
        Err(err) => println!("error:{}", err),
    }

    String::from("ok")
}

#[derive(Debug, Deserialize)]
#[warn(dead_code)]
struct UrlConfig {
    root: String,
    urls: Vec<String>,
}
