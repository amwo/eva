//! Compress HTML/CSS/Javascript

pub fn type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

//use regex::Regex;

#[test]
fn comp() {
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

    println!(
        "{}",
        "==============================================================================="
    );

    //let mut p = 0;
    for c in html.chars() {
        if c == '\n' || c == '\t' || c == '　' {
            c.escape_default();
        } else {
            type_of(&c);
        }
    }

    println!("{}", html);

    println!(
        "{}",
        "==============================================================================="
    );
}
