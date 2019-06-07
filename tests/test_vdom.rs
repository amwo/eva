#[macro_use]
extern crate eva;

use std::collections::HashMap;

struct Vdom {
    e: String, // element name
    t: String, // Text content
    a: String, // Attrs
}

#[test]
fn test_vdom() {
    println!("{}", "==============================================");
    let mut h = HashMap::new();
    h.insert("tag", "html");
    h.insert("text", "Hello !");
    println!("{:?}", h);
    println!("{}", "==============================================");
    //html!(
    //    r#"
    //      <!doctype html>
    //      </html>
    //    "#
    //);
}
