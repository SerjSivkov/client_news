use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article parsing failed")]
    ArticleParseError(serde_json::Error)
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>
}
#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String
}

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))
    ?.into_string().map_err(|e| NewsApiError::FailedResponseToString(e))?;
    let articles = serde_json::from_str::<Articles>(&response).map_err(|e| NewsApiError::ArticleParseError(e))?;

    Ok(articles)
}