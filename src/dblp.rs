use crate::BibtexServer;

pub struct API {}

impl API {
    const URL: &'static str = "https://dblp.org/search/publ/api?format=bibtex&q=";
}

impl BibtexServer for API {
    fn fetch(&self, query: &str) -> String {
        let encoded_query = String::from(urlencoding::encode(query));
        let mut url_query = String::from(API::URL);
        url_query.push_str(&encoded_query);
        let response = reqwest::blocking::get(&url_query)
            .expect(&format!("Cannot load {}", url_query));
        response.text().unwrap().trim().to_owned()
    }
}