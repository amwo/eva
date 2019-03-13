use siva::{Component, props, store, render};

// Import layouts
use components::footer;
use components::header;

// Import components
use components::h1;

pub struct Index {
    props,
}

impl Index {
    // Before render
    fn init() {}

    // After render
    fn renderd() {}

    // Rerender
    fn update() {}

    // Unmount
    fn unmount() {}
}

impl Render<Index> for Index {
    fn h(&self) -> Html<Self> {
        render!(
            r#"
                <header />
                    <div>
                        <h1 title="Index Page" />
                    </div>
                <footer />
            "#
        );
    }
}
