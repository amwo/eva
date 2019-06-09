#[macro_export]
macro_rules! html {
    ($t:tt) => {
        println!("tt: {}", stringify!($t))
    };
}

#[macro_export]
macro_rules! render {
    ($html:expr) => {
        println!("{}", $html);
    };
}

#[macro_export]
macro_rules! css {
    ($html:expr) => {
        println!("{}", $html);
    };
}

#[macro_export]
macro_rules! js {
    ($html:expr) => {
        println!("{}", $html);
    };
}
