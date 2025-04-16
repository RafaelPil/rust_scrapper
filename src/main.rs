use reqwest::Error;
use scraper::{Html, Selector};
use tokio::task;


#[derive(Debug)]
struct ParseData {
    titles: Vec<String>,
    links: Vec<String>,
}

async fn fetch_html(url: String) -> Result<String, Error> {
    let response = reqwest::get(&url).await?;
    response.text().await
}
fn main() {
    println!("Hello, world!");
}
