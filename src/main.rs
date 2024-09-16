#[macro_use]
extern crate actix_web;

use std::{env, io};
use std::num::NonZeroUsize;
use std::sync::{Mutex};
use actix_web::{middleware, App, HttpServer, web};
use actix_web::web::Data;
use lru::LruCache;
use crate::g_news::{Article, Response};

mod constants;
mod g_news;
mod api;



fn init_cache() -> Mutex<LruCache<String, Response<Article>>> {
    Mutex::new(LruCache::new(NonZeroUsize::new(500).unwrap()))
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let article_cache = web::Data::new(init_cache());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::clone(&article_cache))
            .service(g_news::news)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}