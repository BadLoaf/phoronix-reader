use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Name};

pub struct Article {
    pub title: String,
    pub summary: String,
    pub link: String,
}

impl Article {
    pub fn get_articles() -> Vec<Article> {
        Document::from(
            reqwest::blocking::get("https://www.phoronix.com/")
                .expect(
                    "Error: Failed to fetch Phoronix. Check your internet connection and try again",
                )
                .text()
                .expect("Error: Failed to parse text")
                .as_str(),
        )
        .find(Attr("style", "clear: both;"))
        .take(5)
        .map(|node| Article::new(&node))
        .collect()
    }

    fn new(node: &Node) -> Article {
        let header: Node = node.find(Name("a")).next().unwrap();
        let link = header.attr("href").unwrap();
        let summary = textwrap::fill(node.find(Name("p")).next().unwrap().text().as_str(), 80);
        Article {
            title: header.text(),
            link: String::from(link),
            summary,
        }
    }
}
