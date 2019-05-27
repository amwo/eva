//! Compress HTML/CSS/Javascript
use regex::Regex;

pub fn comp(data: &'static str) -> &'static str {
    let r = Regex::new(r"\n|\t|\s+").unwrap();
    println!("{}", r.replace_all(data, ""));
    data
}

// Other utils
pub fn type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}
