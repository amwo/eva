use std::env;

#[test]
fn get_args() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
