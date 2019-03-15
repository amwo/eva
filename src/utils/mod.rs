//! Compress HTML/CSS/Javascript

fn remove_spaces(c: char) -> char {
    if c == '\n' || c == ' ' || c == '\t' || c == '\r' || c == '　' {
        '\u{00}' // Return unicode blank character
    } else {
        c
    }
}

pub fn comp(s: &str) -> String {
    let tags = ["html", "body"];
    let mut p = 0;
    for c in s.chars() {
        if c == '<' {
            p = 1;
        } else {
            p = 0;
        }
    }

    "Hello".to_string()
}

// Backup
//fn remove_spaces(c: char) -> char {
//    if c == '\n' || c == ' ' || c == '\t' || c == '\r' || c == '　' {
//        // Return unicode blank character
//        '\u{00}'
//    } else {
//        c
//    }
//}
//
//pub fn comp(s: &str) -> String {
//    s.chars().map(|c| remove_spaces(c)).collect()
//}

#[test]
fn test_comp() {
    let html = r#"
        <div id="test">
            <!-- normal comment -->
            <div>content</div>
            <!--[if lte IE 8]>
            <div class="warning">This old browser is unsupported and will most likely display funky things.
            </div>
            <![endif]-->
        </div>
    "#;

    println!("{}", "===============================================================================");
    println!("{}", comp(html));
    println!("{}", "===============================================================================");
}

// Other utils
pub fn type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}
