use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

// Structure about json response
#[derive(Debug, Deserialize, Clone)]
pub struct AnswerEntry {
    pub discord: String,
    pub license: String,
    pub license2: String,
    pub xbl: String,
    pub fivem: String,
    pub steam: String,
    pub live: String,
    pub pseudo: String,
    pub date: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchResult {
    pub time: u64,
    pub value: u64,
    pub answer: HashMap<String, AnswerEntry>,
}

// Search with SearchHub API
pub fn search_query(query: &str, api_key: &str) -> Result<SearchResult, Box<dyn Error>> {
    let url = "https://api.searchhub.ru/search";
    search_query_with_url(query, api_key, url)
}

// Request to SearchHub API to fetch response for FiveM scraps
fn search_query_with_url(query: &str, api_key: &str, url: &str) -> Result<SearchResult, Box<dyn Error>> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("x-api-key", HeaderValue::from_str(api_key)?);

    let payload = serde_json::json!({ "query": query });

    let response = client
    .post(url)
    .headers(headers)
        .json(&payload)
    .send()?
        .error_for_status()?;

    let parsed = response.json::<SearchResult>()?;
    Ok(parsed)
}