use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

trait BibtexServer {
    fn fetch(&self, query: String) -> String;
}

struct DBLP {}

impl DBLP {
    const URL: &'static str = "https://dblp.org/search?q=";
}

impl DBLP {
    fn fetch_bibtex(&self, url: &str) -> String {
        let response = reqwest::blocking::get(url)
            .expect(&format!("Cannot load {}", url));
        let html_content = response.text().unwrap();
        let document = Document::from(html_content.as_str());

        let bib_tex = document.find(Attr("id", "bibtex-section").descendant(Name("pre")))
            .next()
            .expect("Cannot find bib_tex node. Maybe the page format have been changed.");

        bib_tex.text()
    }
}

impl BibtexServer for DBLP {
    fn fetch(&self, query: String) -> String {
        let entry_selector = Name("ul").and(Class("publ-list")).child(Name("li").and(Class("entry")));
        let bibtex_link_selector = Class("body").child(Name("ul")).child(Name("li")).child(Name("a").and(Attr("href", ())));

        let encoded_query = String::from(urlencoding::encode(&query));

        let mut url_query = String::from(DBLP::URL);
        url_query.push_str(&encoded_query);
        let response = reqwest::blocking::get(&url_query)
            .expect(&format!("Cannot load {}", url_query));
        let html_content = response.text().unwrap();

        let mut entries = String::new();
        let document = Document::from(html_content.as_str());
        for entry in document.find(entry_selector) {
            for a in entry.find(bibtex_link_selector) {
                if let Some(url) = a.attr("href") {
                    if url.ends_with("bibtex") {
                        entries.push_str(self.fetch_bibtex(url).trim());
                        entries.push('\n');
                    }
                }
            }
        }
        entries
    }
}

fn main() {
    let dblp_server = DBLP{};
    for query in std::env::args() {
        println!("{}", dblp_server.fetch(query));
    }
}