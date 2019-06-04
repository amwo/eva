use eva::{html, Component, Html, Renderable};

pub struct H1;

pub enum Action {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Slider {
    fn update(&mut self, action: Self::Action) -> ShouldRender {
        match action {
            Action::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Action::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Action::Bulk(list) => {
                for msg in list {
                    self.update(msg);
                    self.console.log("Bulk action");
                }
            }
        }
        true
    }
}

impl Render<H1> for H1 {
    fn view(&self) -> Html<Self> {
        html! (#"
            <h1></h1>
        "#);
    }
}
