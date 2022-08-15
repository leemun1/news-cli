use std::error::Error;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles  {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}
fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();
    let url= format!("https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey={}", api_key);
    let articles = get_articles(&url);

    dbg!(articles);
}
