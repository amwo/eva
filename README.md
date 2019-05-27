# Eva
Eva is a Rust library for building static site.

- **Static Site Generator:** Eva generates simple static html files. That is fastest way in web.
- **Components:** Generate html from your components system. That is new concept.
- **WebAssembly:** A part of javascript works WebAsssembly.

## Installation
### Requires
For all Eva developer.

    $ cargo install cargo-dev

If you need REST API.

    $ cargo install cargo-rest

If you need CLI static site generator.

    $ cargo install cargo-gen

### Configuration
```Rust:Cargo.toml
...

[dependencies]
eva = "0.1.0"

[eva]
db = "gRPC"
db-host = ""
db-user = ""
db-pass = ""
dev-host = "localhost"
dev-port = "8181"
production-host = "0.0.0.0"
production-port = "5000"
ftp-host = "0.0.0.0"
ftp-user = "ftp-user"
ftp-pass = "xxxxx"
public = "public"
```

## Usage
### Command Line Tool
### Rest API Server



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

## Test
    cargo test -- --nocapture

## License
Eva is MIT licensed.
