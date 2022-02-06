use scraper;
use std::thread;
use tokio;

mod scraping;

#[tokio::main]
async fn main() -> Result<(), ()> {
    // let result = scraping::fetch_meta_fields("https://zenn.dev/mayo_dev".to_string()).await;
    // tokio::task::spawn_blocking(|| {
    //     scraping::fetch_meta_fields("https://zenn.dev/mayo_dev".to_string());
    // })
    // .await
    // .expect("Task panicked");

    thread::spawn(|| {
        scraping::fetch_meta_fields("https://zenn.dev/mayo_dev".to_string());
    })
    .join()
    .expect("Thread panicked");

    // println!("{:?}", result);
    Ok(())
}

// selectは使わずscraperを使う
async fn get_meta_tags(url: String) -> Result<String, ()> {
    let body = reqwest::get(&url).await.unwrap().text().await.unwrap();
    let document = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse("meta").unwrap();
    let elements = document.select(&selector);
    println!("{:?}", elements);

    Ok(body)
}
