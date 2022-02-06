// use scraper;

// // #[derive(Debug)]
// pub struct MetaFields {
//   title: String,
//   description: String,
//   image: String,
// }

// #[tokio::main]
// pub async fn fetch_meta_fields(url: String) -> Result<MetaFields, ()> {
//   let body = reqwest::get(&url).await.unwrap().text().await.unwrap();

//   let fragment = scraper::Html::parse_fragment(&body);

//   let title_selector = scraper::Selector::parse("title").unwrap();

//   let (og_title_selector, og_description_selector, og_image_selector) = get_meta_selectors();

//   let mut meta_fields = MetaFields {
//     title: "".to_string(),
//     description: "".to_string(),
//     image: "".to_string(),
//   };

//   let mut result_str = "".to_string();
//   for element in fragment.select(&og_title_selector) {
//     meta_fields.title = element.text().collect::<Vec<_>>().join("");
//     result_str += &element.text().collect::<Vec<_>>().join("");
//     result_str += "\n";
//   }
//   for element in fragment.select(&og_description_selector) {
//     meta_fields.description = element.text().collect::<Vec<_>>().join("");
//     result_str += &element.text().collect::<Vec<_>>().join("");
//     result_str += "\n";
//   }
//   for element in fragment.select(&og_image_selector) {
//     meta_fields.image = element.text().collect::<Vec<_>>().join("");
//     result_str += &element.text().collect::<Vec<_>>().join("");
//     result_str += "\n";
//   }

//   println!("{}", meta_fields.title);

//   Ok(meta_fields)
//   // Ok(result_str)
// }

// fn get_meta_selectors() -> (scraper::Selector, scraper::Selector, scraper::Selector) {
//   let (title, description, image) = ("title", "description", "image");
//   // let selector_temp = r#"meta[property="{}"]"#;
//   (
//     scraper::Selector::parse(&format!(r#"meta[property="{}"]"#, title)).unwrap(),
//     scraper::Selector::parse(&format!(r#"meta[property="{}"]"#, description)).unwrap(),
//     scraper::Selector::parse(&format!(r#"meta[property="{}"]"#, image)).unwrap(),
//   )
// }

use scraper;

#[tokio::main]
pub async fn fetch_meta_fields(url: String) -> Result<String, ()> {
  let body = reqwest::get(&url).await.unwrap().text().await.unwrap();

  let fragment = scraper::Html::parse_fragment(&body);

  // let (og_title_selector, og_description_selector, og_image_selector) = get_meta_selectors();

  let mut result_str = "".to_string();
  // for element in fragment.select(&og_title_selector) {
  //   result_str += &element.text().collect::<Vec<_>>().join("");
  //   result_str += "\n";
  // }
  // for element in fragment.select(&og_description_selector) {
  //   result_str += &element.text().collect::<Vec<_>>().join("");
  //   result_str += "\n";
  // }
  // for element in fragment.select(&og_image_selector) {
  //   result_str += &element.text().collect::<Vec<_>>().join("");
  //   result_str += "\n";
  // }

  println!("{:?}", fragment);
  println!("{}", result_str);
  Ok(result_str)
}

fn get_meta_selectors() -> (scraper::Selector, scraper::Selector, scraper::Selector) {
  let (title, description, image) = ("title", "description", "image");
  // let selector_temp = r#"meta[property="{}"]"#;
  (
    scraper::Selector::parse(&format!(r#"meta[property="{}"]"#, title)).unwrap(),
    scraper::Selector::parse(&format!(r#"meta[property="{}"]"#, description)).unwrap(),
    scraper::Selector::parse(&format!(r#"meta[property="{}"]"#, image)).unwrap(),
  )
}
