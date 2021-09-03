use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use yew::{html, Component, ComponentLink, Html, MouseEvent, ShouldRender};

use crate::fetcher::Fetcher;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub id: String,
    pub name: String,
}

type WorldId = String;
type Location = String;
const PRIVATE_LOCATION: &'static str = "private";

pub struct Model {
    link: ComponentLink<Self>,
    counter: i32,
    firends: Vec<Friend>,
    favorites: Vec<Favorite>,
    favorte_friends: HashMap<Location, Vec<Friend>>,
    worlds: HashMap<WorldId, World>,
    fetcher: Fetcher,
}

pub enum Msg {
    DidFetchFriends(Vec<Friend>),
    DidFetchFavorites(Vec<Favorite>),
    DidFetchWorld(World),
    Reload(MouseEvent),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut me = Self {
            link,
            counter: 0,
            firends: vec![],
            favorites: vec![],
            favorte_friends: HashMap::new(),
            worlds: HashMap::new(),
            fetcher: Fetcher::new(),
        };
        me.fetch_favorites();
        me
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DidFetchFriends(friends) => {
                self.firends = friends;
                self.pick_up_favorite_friends();
                true
            }
            Msg::DidFetchFavorites(favorites) => {
                self.favorites = favorites;
                self.fetch_friends();
                false
            }
            Msg::DidFetchWorld(world) => {
                self.worlds.insert(world.id.clone(), world);
                true
            }
            Msg::Reload(event) => {
                event.prevent_default();
                self.fetch_friends();
                true
            }
        }
    }

    fn view(&self) -> Html {
        let reload = self.link.callback(Msg::Reload);
        html! {
          <div>
            <div><button onclick=reload>{"reload"}</button></div>
            <div>{"に～ぼし"}</div>
            {self.view_favorte_friends()}
            <div>{"に～ぼし"}</div>
            <div class="friends">{for self.firends.iter().map(|x| self.view_friend(x))}</div>
            <div>{self.counter}</div>
          </div>
        }
    }
}

impl Model {
    fn fetch_favorites(&mut self) {
        self.fetcher.get(
            "https://vrchat.com/api/1/favorites?n=100&type=friend",
            self.link.callback(Msg::DidFetchFavorites),
        );
    }

    fn fetch_friends(&mut self) {
        self.fetcher.get(
            "https://vrchat.com/api/1/auth/user/friends",
            self.link.callback(Msg::DidFetchFriends),
        );
    }

    fn fetch_world(&mut self, world_id: &str) {
        self.fetcher.get(
            &format!("https://vrchat.com/api/1/worlds/{}", world_id),
            self.link.callback(Msg::DidFetchWorld),
        );
    }

    fn pick_up_favorite_friends(&mut self) {
        let mut i = 0;
        while i < self.firends.len() {
            if self
                .favorites
                .iter()
                .any(|favorite| favorite.favorite_id == self.firends[i].id)
            {
                let friend = self.firends.remove(i);
                self.favorte_friends
                    .entry(friend.location.clone())
                    .or_default()
                    .push(friend);
            } else {
                i += 1;
            }
        }
        let mut world_ids = HashSet::new();
        for location in self.favorte_friends.keys() {
            if location == PRIVATE_LOCATION {
                continue;
            }
            if let Some(world_id) = location_to_world_id(location) {
                if !self.worlds.contains_key(&world_id) {
                    world_ids.insert(world_id.to_string());
                }
            }
        }
        for world_id in world_ids {
            self.fetch_world(&world_id);
        }
    }

    fn view_favorte_friends(&self) -> Html {
        let xs = self.favorte_friends.iter().map(|(location, friends)| {
            let world_id = location_to_world_id(location);
            let world = if let Some(world_id) = world_id {
                if let Some(world) = self.worlds.get(&world_id) {
                    html! {
                      <div>{&world.name}</div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            };
            html! {
              <div>
                {world}
                {for friends.iter().map(|friend| self.view_friend(friend))}
              </div>
            }
        });
        html! {for xs}
    }

    fn view_friend(&self, friend: &Friend) -> Html {
        html! {
          <a href="#">
            <img src=friend.current_avatar_thumbnail_image_url.clone() />
            <div>{&friend.display_name}</div>
          </a>
        }
    }
}

fn location_to_world_id(location: &str) -> Option<String> {
    location.split(":").next().map(|x| x.to_string())
}
