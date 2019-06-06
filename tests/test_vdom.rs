#[macro_use]
extern crate eva;

use std::collections::HashMap;

static E: &'static [&str] = &[
    // HTML5
    "a",
    "abbr",
    "address",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "bdi",
    "bdo",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "cite",
    "col",
    "colgroup",
    "data",
    "datalist",
    "div",
    "dd",
    "dl",
    "dt",
    "em",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hr",
    "html",
    "i",
    "iframe",
    "img",
    "input",
    "ins",
    "kbd",
    "label",
    "legend",
    "li",
    "link",
    "main",
    "map",
    "mark",
    "meta",
    "meter",
    "nav",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "pre",
    "progress",
    "q",
    "rp",
    "rt",
    "ruby",
    "s",
    "samp",
    "script",
    "section",
    "select",
    "small",
    "source",
    "span",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
];

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
