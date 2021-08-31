use anyhow::Error;
use serde::Deserialize;
use yew::{
    format::{Json, Nothing},
    html,
    services::{
        fetch::{FetchTask, Request, Response},
        FetchService,
    },
    Component, ComponentLink, Html, MouseEvent, ShouldRender,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
}

pub struct Model {
    link: ComponentLink<Self>,
    counter: i32,
    users: Vec<User>,
    _fetch_task: FetchTask,
}

pub enum Msg {
    DidFetchFriends(Response<Json<Result<Vec<User>, Error>>>),
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
            users: vec![],
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
                        Ok(users) => {
                            self.users = users;
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
            <div>{for self.users.iter().map(|x| self.view_user(x))}</div>
            <div>{self.counter}</div>
            <div><button onclick=click>{"++"}</button></div>
          </div>
        }
    }
}

impl Model {
    fn view_user(&self, user: &User) -> Html {
        html! {
          <div>
            <div>{&user.id}</div>
            <div>{&user.display_name}</div>
          </div>
        }
    }
}
