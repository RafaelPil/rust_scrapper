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

fn parse_html(html: &str) -> ParseData {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("h1, h2, h3").unwrap();
    let links_selector = Selector::parse("a").unwrap();

    let titles = document
        .select(&title_selector)
        .map(|element| element.text().collect::<Vec<_>>().join(""))
        .collect();

    let links = document
        .select(&links_selector)
        .filter_map(|element| element.value().attr("href"))
        .map(String::from)
        .collect();

    ParseData {titles, links}
}

fn main() {
    println!("Hello, world!");
}
