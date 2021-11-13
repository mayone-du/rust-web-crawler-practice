use select::document::Document;
use select::predicate::Name;
use std::env;

fn main() -> eyre::Result<()> {
    let args: Vec<String> = env::args().collect();

    let url = if args.len() > 1 {
        &args[1]
    } else {
        "https://www.rust-lang.org/en-US/downloads.html"
    };

    let body = reqwest::blocking::get(url)?.text()?;

    let doc = Document::from(body.as_str());
    for href in doc.find(Name("meta")).filter_map(|a| a.attr("content")) {
        println!("{:?}", href);
    }

    Ok(())
}
