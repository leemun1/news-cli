use std::error::Error;
extern crate dotenv;

use dotenv::dotenv;
use std::env;


struct Articles  {
    articles: Vec<Article>,
}

struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    println!("yooo {}", response);

    dbg!(response);

    todo!()
}
fn main() {
    dotenv().ok();

    let api_key = env::var("API_KEY");
    let url= format!("https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey={:?}", api_key);
    let articles = get_articles(&url);
}
