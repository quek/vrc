use anyhow::Error;
use std::ops::Deref;
use yew::callback::Callback;
use yew::format::Nothing;
use yew::format::{Json, Text};
use yew::services::fetch;
use yew::services::Task;

type Request<T> = fetch::Request<T>;
type Response<T> = fetch::Response<Json<Result<T, Error>>>;

struct FetchTask(fetch::FetchTask);

fn delete<OUT>(url: &str, callback: Callback<OUT>) -> FetchTask
where
    Json<Result<OUT, Error>>: From<Text> + 'static,
{
    let request = Request::delete(url)
        .header("Content-Type", "application/json")
        .body(Nothing)
        .unwrap();
    fetch(request, callback)
}

fn get<OUT>(url: &str, callback: Callback<OUT>) -> FetchTask
where
    Json<Result<OUT, Error>>: From<Text> + 'static,
{
    let request = Request::get(url)
        .header("Content-Type", "application/json")
        .body(Nothing)
        .unwrap();
    fetch(request, callback)
}

fn post<IN, OUT>(url: &str, post_data: IN, callback: Callback<OUT>) -> FetchTask
where
    Json<IN>: Into<Text>,
    Json<Result<OUT, Error>>: From<Text> + 'static,
{
    let request = Request::post(url)
        .header("Content-Type", "application/json")
        .body(Json(post_data))
        .unwrap();
    fetch(request, callback)
}

fn put<IN, OUT>(url: &str, post_data: IN, callback: Callback<OUT>) -> FetchTask
where
    Json<IN>: Into<Text>,
    Json<Result<OUT, Error>>: From<Text> + 'static,
{
    let request = Request::put(url)
        .header("Content-Type", "application/json")
        .body(Json(post_data))
        .unwrap();
    fetch(request, callback)
}

fn fetch<IN, OUT>(request: fetch::Request<IN>, callback: Callback<OUT>) -> FetchTask
where
    IN: Into<Text>,
    Json<Result<OUT, Error>>: From<Text> + 'static,
{
    let callback = Callback::once(move |input: Response<OUT>| {
        let (meta, Json(json_data)) = input.into_parts();
        if meta.status.is_success() {
            match json_data {
                Ok(out) => {
                    callback.emit(out);
                }
                Err(error) => {
                    handle_api_error(Some(error));
                }
            }
        } else if meta.status.as_u16() == 401 {
            handle_api_error(None);
        } else if meta.status.as_u16() == 408 {
            // FetchError::Canceled
            // nothing to do
        } else {
            handle_api_error(None);
        }
    });
    fetch_impl(request, callback)
}

fn fetch_impl<IN, OUT: 'static>(
    request: fetch::Request<IN>,
    callback: Callback<fetch::Response<OUT>>,
) -> FetchTask
where
    IN: Into<Text>,
    OUT: From<Text>,
{
    let x = Callback::from(move |input| callback.emit(input));
    FetchTask(fetch::FetchService::fetch(request, x).unwrap())
}

impl Task for FetchTask {
    fn is_active(&self) -> bool {
        self.0.is_active()
    }
}

impl Drop for FetchTask {
    fn drop(&mut self) {
        if self.is_active() {}
    }
}

impl Deref for FetchTask {
    type Target = fetch::FetchTask;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Fetcher {
    tasks: Vec<FetchTask>,
}

impl Fetcher {
    pub fn new() -> Self {
        Self { tasks: vec![] }
    }

    pub fn delete<OUT>(&mut self, url: &str, callback: Callback<OUT>)
    where
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        self.push_task(delete(url, callback));
    }

    pub fn get<OUT>(&mut self, url: &str, callback: Callback<OUT>)
    where
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        self.push_task(get(url, callback));
    }

    pub fn is_active(&self) -> bool {
        self.tasks.iter().any(|task| task.is_active())
    }

    pub fn post<IN, OUT>(&mut self, url: &str, post_data: IN, callback: Callback<OUT>)
    where
        Json<IN>: Into<Text>,
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        self.push_task(post(url, post_data, callback));
    }

    pub fn put<IN, OUT>(&mut self, url: &str, post_data: IN, callback: Callback<OUT>)
    where
        Json<IN>: Into<Text>,
        Json<Result<OUT, Error>>: From<Text> + 'static,
    {
        self.push_task(put(url, post_data, callback));
    }

    fn push_task(&mut self, fetch_task: FetchTask) {
        self.tasks.retain(|x| x.is_active());
        self.tasks.push(fetch_task);
    }
}

fn handle_api_error(error: Option<Error>) {
    if let Some(error) = error {
        web_sys::console::error_1(&format!("{:?}", error).into());
    }
}
