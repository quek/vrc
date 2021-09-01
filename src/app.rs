use serde::Deserialize;
use yew::{html, Component, ComponentLink, Html, MouseEvent, ShouldRender};

use crate::fetcher::Fetcher;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub current_avatar_thumbnail_image_url: String,
    pub location: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Favorite {
    pub id: String,
    pub favorite_id: String,
}

pub struct Model {
    link: ComponentLink<Self>,
    counter: i32,
    users: Vec<User>,
    favorites: Vec<Favorite>,
    _fetcher: Fetcher,
}

pub enum Msg {
    DidFetchFriends(Vec<User>),
    DidFetchFavorites(Vec<Favorite>),
    Click(MouseEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut fetcher = Fetcher::new();
        fetcher.get(
            "https://vrchat.com/api/1/auth/user/friends",
            link.callback(Msg::DidFetchFriends),
        );
        fetcher.get(
            "https://vrchat.com/api/1/favorites?n=100&type=friend",
            link.callback(Msg::DidFetchFavorites),
        );
        Self {
            link,
            counter: 0,
            users: vec![],
            favorites: vec![],
            _fetcher: fetcher,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DidFetchFriends(users) => {
                self.users = users;
                true
            }
            Msg::DidFetchFavorites(favorites) => {
                self.favorites = favorites;
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
            <div class="users">{for self.users.iter().map(|x| self.view_user(x))}</div>
            <div>{self.counter}</div>
            <div><button onclick=click>{"++"}</button></div>
          </div>
        }
    }
}

impl Model {
    fn view_user(&self, user: &User) -> Html {
        html! {
          <a href="#">
            <img src=user.current_avatar_thumbnail_image_url.clone() />
            <div>{&user.display_name}</div>
          </a>
        }
    }
}
