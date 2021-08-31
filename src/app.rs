use anyhow::Error;
use yew::{
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, Html, MouseEvent, ShouldRender,
};

pub struct Model {
    link: ComponentLink<Self>,
    counter: i32,
    foo: String,
    _fetch_task: FetchTask,
}

pub enum Msg {
    DidFetchFriends(Response<Json<Result<String, Error>>>),
    Click(MouseEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let request = Request::get("https://vrchat.com/api/1/auth/user/friends")
            .body(Nothing)
            .unwrap();
        let callback = link.callback(Msg::DidFetchFriends);
        let _fetch_task = FetchService::fetch(request, callback).unwrap();
        Self {
            link,
            counter: 0,
            foo: "ふぇっち".to_string(),
            _fetch_task,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DidFetchFriends(response) => {
                let (meta, Json(json_data)) = response.into_parts();
                if meta.status.is_success() || meta.status.as_u16() == 304 {
                    match json_data {
                        Ok(out) => {
                            self.foo = out;
                        }
                        Err(_error) => {
                            // utils::handle_api_error(Some(error));
                        }
                    }
                } else {
                    // utils::handle_api_error(None);
                }
                true
            }
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
            <div>{&self.foo}</div>
            <div>{self.counter}</div>
            <div><button onclick=click>{"++"}</button></div>
          </div>
        }
    }
}
