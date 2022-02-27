use scraper;
use std::thread;
use tokio;
use reqwest;

mod scraping;

// #[tokio::main]
fn main() -> Result<(), ()> {
    let body = reqwest::blocking::get("https://www.youtube.com/watch?v=v9V5aByfeCM")
        .unwrap()
        .text()
        .unwrap();
    // println!("{}", body);
    
    let fragment = scraper::Html::parse_fragment(&body);
    let og_title_selector = scraper::Selector::parse(r#"meta[property="og:title"]"#).unwrap();
    let favicon_selector = scraper::Selector::parse(r#"link[rel="icon"]"#).unwrap();
    let meta_title_selector = scraper::Selector::parse("title").unwrap();

    for element in fragment.select(&favicon_selector) {
        let favicon_url = element.value().attr("href").unwrap();
        println!("favicon url is {}", favicon_url);
    }

    // let mut meta_title = String::new();
    for element in fragment.select(&og_title_selector) {
        let og_title = element.value().attr("content");
        println!("og title is {:?}", og_title);
        // {
            // Some(content) => content.to_string(),
            // None => {
            //     let mut title = "".to_string();
            //     for title_tag in fragment.select(&meta_title_selector) {
            //         title += title_tag.text().collect::<Vec<_>>().join(" ").as_str();
            //     }
            //     println!("{}", title);
            //     title
            // }
        // };
    }
    
        let mut title = "".to_string();
        for title_tag in fragment.select(&meta_title_selector) {
            title += title_tag.text().collect::<Vec<_>>().join(" ").as_str();
        }
        println!("meta title is {}", title);

            //     println!("{}", title);
    // scraping::fetch_meta_fields("https://mayone-du.github.io/yew-blog/".to_string());
    // scraping::fetch_meta_fields("https://zenn.dev/mayo_dev".to_string());
    // tokio::task::spawn_blocking(|| {
    //     scraping::fetch_meta_fields("https://zenn.dev/mayo_dev".to_string());
    // })
    // .await
    // .expect("Task panicked");

    // thread::spawn(|| {
    //     scraping::fetch_meta_fields("https://zenn.dev/mayo_dev".to_string());
    // })
    // .join()
    // .expect("Thread panicked");

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
