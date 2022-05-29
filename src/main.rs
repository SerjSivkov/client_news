mod theme;

use std::error::Error;
use dotenv::dotenv;
use newsapi::{Article, Country, NewsAPI, Endpoint};
// use newsapi::{get_articles, Articles};

fn render_articles(articles: &Vec<Article>) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in articles {
        theme.print_text(&format!("`{}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::Ru);

    let newsapi_response = newsapi.fetch()?;

    render_articles(&newsapi_response.articles());

    Ok(())
}
