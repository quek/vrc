use yew::{html, Component, ComponentLink, Html, MouseEvent, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    counter: i32,
}

pub enum Msg {
    Click(MouseEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, counter: 0 }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(event) => {
                event.prevent_default();
                self.counter += 1;
                true
            }
        }
    }

    fn view(&self) -> Html {
        let click = self.link.callback(Msg::Click);
        html! {
          <div>
            <div>{"に～ぼし"}</div>
            <div>{self.counter}</div>
            <div><button onclick=click>{"++"}</button></div>
          </div>
        }
    }
}
