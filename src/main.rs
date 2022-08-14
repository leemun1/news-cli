use std::error::Error;

struct Articles  {
    articles: Vec<Article>,
}

struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    dbg!(response);

    todo!()
}
fn main() {
    let url= "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=API_KEY";
    let articles = get_articles(url);
}
