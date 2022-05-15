use std::error::Error;
use colour::{dark_green, yellow};
use newsapi::{Articles, get_articles};

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let url: &str = "https://newsapi.org/v2/top-headlines?country=ru&sortBy=publishedAt&apiKey=85230bc9d0ef479f9d1e49e1b06e226e";
    let articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())
}
