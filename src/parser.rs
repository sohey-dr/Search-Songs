use scraper::{Html, Selector};
use std::collections::HashMap;

pub struct Parser {}

impl Parser {
    pub async fn search_lyric(body: &str) -> Vec<HashMap<&str, String>> {
        let document = Html::parse_document(&body);
        let selector = Selector::parse(".kCrYT > a").unwrap();

        let mut links = vec![];
        for node in document.select(&selector) {
            let href = node.value().attr("href").unwrap();

            // モジュールであるscraperの場合Googleをスクレイピングすると無駄な文字列が入るので削除
            let last_index = href.find("&sa=").unwrap();
            let url = &href[7..last_index];

            if url.contains("uta-net") || url.contains("j-lyric.net") || url.contains("utamap") {
                let mut link = HashMap::new();
                link.insert("title", node.text().collect::<Vec<_>>()[0].to_string());
                link.insert("url", url.to_string());
                links.push(link);
            }
        }

        links
    }
}