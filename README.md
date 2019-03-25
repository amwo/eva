# Eva
Eva is a Rust library for building static site.

- **Static Site Generator:** Eva generates simple static html files. That is fastest way in web.
- **Components:** Generate html from your components system. That is new concept.
- **WebAssembly:** A part of javascript works WebAsssembly.

## Installation
```Rust:Cargo.toml
[dependencies]
eva = "0.1.0"
```

## Documentation
wip..

## Example
Here is the first one to get you started:
```Rust:src/main.rs
// This is a Draft
use eva::{Component, Html, Render, States};

struct Props {}

impl Component for Props {
    fn create() {
    }
    fn mounted() {
    }
}

impl Render<Props> for Props {
    fn view(&self) -> Html<Self> {
        render! {
            <div>
                <p>Hello, World!</p>
            </div>
        }
    }
}

fn main() {
    eva::app::<Component>();
}
```

## Getting Started

    $ cargo dev

To loanched develop server with hot reloading.

## Production build

    $ cargo gen

Generate to html website into `/public/`.

## License
Eva is MIT licensed.
