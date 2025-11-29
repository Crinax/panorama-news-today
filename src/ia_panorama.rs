use anyhow::anyhow;
use reqwest::blocking::get;
use scraper::{Html, Selector};

pub fn get_news(date: &str) -> anyhow::Result<Vec<String>> {
    let body = get(format!("https://panorama.pub/news/{date}"))?.text()?;
    let document = Html::parse_document(&body);

    let selector = Selector::parse("a.flex[href^='/news'] div:nth-child(3)")
        .map_err(|err| anyhow!("{err}"))?;

    let mut news: Vec<String> = Vec::new();

    for element in document.select(&selector) {
        news.push(element.text().collect());
    }

    let news: Vec<String> = news
        .iter()
        .map(|header| header.trim_ascii().to_owned() + ".")
        .collect();

    Ok(news)
}
