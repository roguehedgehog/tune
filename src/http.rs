use std::collections::HashMap;
use url::form_urlencoded::Serializer;
pub trait Query {
    fn get_params(&self) -> HashMap<String, String>;

    fn get_query(&self) -> String {
        let mut serializer = Serializer::new(String::new());
        for (k, v) in &self.get_params() {
            serializer.append_pair(&k.to_owned(), &v.to_owned());
        }

        serializer.finish()
    }

    fn get_url(&self, endpoint: &str) -> String {
        format!("{}?{}", endpoint, self.get_query())
    }
}
