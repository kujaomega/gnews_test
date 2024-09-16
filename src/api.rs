use failure::Error;

use crate::g_news::{Article, Response};
use urlencoding::encode;
use crate::constants::API_KEY;


pub fn search(n: &Option<usize>, title: &Option<String>, keywords: &Option<String>) -> Result<Response<Article>, Error> {
    let mut url = format!("https://gnews.io/api/v4/search?q=MacRumors&apikey={API_KEY}");
    if n.is_some() {
        let size = n.unwrap();
        url = format!("{url}&max={size}");
    }
    if title.is_some() {
        let title_search = title.clone().unwrap();
        let encoded_title = encode(&title_search);
        url = format!("{url}&in=title&q={encoded_title}");
    } else if keywords.is_some() {
        let keywords_search = keywords.clone().unwrap();
        let encoded_keywords = encode(&keywords_search);
        url = format!("{url}&in=title,description,content&q={encoded_keywords}");
    }

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let res = client.get(url)
        .send()?
        .json::<Response<Article>>()?;
    Ok(res)
}