#[macro_export]
macro_rules! html {
    ($rh:expr) => {
        println!("{}", $rh)
    };
}

#[macro_export]
macro_rules! render {
    ($html:expr) => {
        println!("{}", $html);
    };
}
