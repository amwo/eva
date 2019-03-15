struct Vdom {}

impl Vdom {
    pub fn init() {}

    pub fn diff(self) {
        println!("{}", self);
    }

    pub fn patch(self) {}
}
