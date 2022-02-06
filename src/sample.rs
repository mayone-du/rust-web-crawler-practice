use scraper;
use select::document::Document;
use select::predicate::Name;
use std::env;

mod scraping;

fn main() -> eyre::Result<()> {
  let args: Vec<String> = env::args().collect();

  let url = if args.len() > 1 {
    &args[1]
  } else {
    // "https://www.rust-lang.org/en-US/downloads.html"
    "https://zenn.dev/mayo_dev"
    // "https://zenn.dev/"
    // "https://mayone-du.github.io/yew-blog"
  };

  let body = reqwest::blocking::get(url)?.text()?;

  let fields = ("title", "description", "image");
  let fragment = scraper::Html::parse_fragment(&body);

  let title_selector = scraper::Selector::parse("title").unwrap();
  let og_description_selector =
    scraper::Selector::parse(r#"meta[property="og:description"]"#).unwrap();
  for element in fragment.select(&title_selector) {
    println!("{}", element.text().collect::<String>());
  }
  for element in fragment.select(&og_description_selector) {
    println!("{}", element.value().attr("content").unwrap());
  }

  let doc = Document::from(body.as_str());
  // for href in doc.find(Name("meta")).filter_map(|a| a.attr("content")) {
  //     println!("{:?}", href);
  // }
  // metaタグを取得
  let mut meta_tags = doc.find(Name("meta"));
  // let og_tags: Vec<&str> = meta_tags
  //     // .filter(|tag| tag.attr("content").unwrap().starts_with("og:"))
  //     .filter_map(|tag| {
  //         // println!("{:?}", tag.attr("content"));
  //         tag.attr("property")
  //     })
  //     .filter(|tag| {
  //         println!("{:?}", tag);
  //         tag.starts_with("og:")
  //     })
  //     .collect();
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
