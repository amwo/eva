use eva::{html, Component, Html, Renderable};

struct Root {}

impl Component for Root {}

impl CSS for Root {
    css!({
        .title {
            font-size: 33px;
        }
    });
}

impl Render<Root> for Root {
    render!({
        html!({
            <h1 class="title">Hello!</h1>
            <button onclick="increment"></button>
        });
    });
}
