# GNews API
This is a simple API that interacts with the public news API for fetching articles. It's created in Rust.

# Prerequisites
- Install Docker
- Set your API key in src/constants

## Build with
`docker build -t gnews .`

## Run with:
`docker run -ti -p 9090:9090 gnews`

Access `http://localhost:9090/news?` endpoint and use query params like `n` to fetch N news articles, find certain article titles with `title` query param, or find articles that contain certain keywords with `keywords` query param.