use std::fmt::{Display, Formatter};
use std::{thread};
use std::sync::Mutex;
use actix_web::{HttpResponse, web};
use chrono::{DateTime, Utc};
use lru::LruCache;
use serde::{Deserialize, Serialize};
use crate::api::search;
use crate::constants::APPLICATION_JSON;

// pub type Articles = Response<Article>;
type ArticleCache = Mutex<LruCache<String, Response<Article>>>;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Source {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Article {
    pub title: String,
    pub description: String,
    pub content: String,
    pub url: String,
    pub image: String,
    #[serde(rename(serialize = "publishedAt", deserialize = "publishedAt"))]
    pub published_at: DateTime<Utc>,
    pub source: Source,
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response<T> {
    #[serde(rename(serialize = "totalArticles", deserialize = "totalArticles"))]
    pub total_articles: usize,
    pub articles: Vec<T>,
}

#[derive(Deserialize)]
struct ListN {
    n: Option<usize>,
    title: Option<String>,
    keywords: Option<String>
}

#[get("/news")]
pub async fn news(cache: web::Data<ArticleCache>, query_params: web::Query<ListN>) -> HttpResponse {
    let n = match &query_params.n {
        Some(value) => value,
        None => &10,
    };
    let title = match &query_params.title {
        Some(value) => value,
        None => "",
    };
    let keywords = match &query_params.keywords {
        Some(value) => value,
        None => "",
    };
    let cache_key = format!("{n}{title}{keywords}");

    let mut cache_guard = cache.lock().unwrap();

    if let Some(articles) = cache_guard.get(&cache_key) {
        return HttpResponse::Ok().json(articles);
    }


    let (sender, receiver) = std::sync::mpsc::channel();
    thread::spawn(move || {
        let result = search(&query_params.n, &query_params.title, &query_params.keywords);
        sender.send(result).unwrap()
    });

    let received = receiver.recv().unwrap().unwrap();

    cache_guard.put(cache_key, received.clone());

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(received)
}