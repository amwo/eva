//! Compress HTML/CSS/Javascript
use regex::Regex;

pub fn comp(data: &'static str) -> &'static str {
    let r = Regex::new(r"\n|\t|\s+").unwrap();
    println!("{}", r.replace_all(data, ""));
    data
}

pub fn html_to_vdom(data: &'static str) -> &'static str {
    let r = Regex::new(r"\n|\t|\s+").unwrap();
    println!("{}", r.replace_all(data, ""));
    data
}
