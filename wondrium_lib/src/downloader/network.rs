use serde::de::DeserializeOwned;
use std::io::Read;

pub fn get_json_from_url<T: DeserializeOwned>(url: &str) -> T {
        let response = reqwest::blocking::get(url).expect("Unable to GET from the requested url");
        response.json::<T>().expect("Unable to deserialize to json")
}

pub fn get_html_from_url(url: &str) -> String {
        let mut response = reqwest::blocking::get(url).expect("Unable to GET from the requested url");
        let mut html = String::new();
        response.read_to_string(&mut html).expect("Unable to read body of response");
        html
}
