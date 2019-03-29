extern crate serde_json as jsons;

mod utils;

use cfg_if::cfg_if;
use futures::{future, Future};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
use js_sys::Promise;
use web_sys::{Response, Storage};
use std::collections::HashMap;


cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub pub_date: Option<String>,
    pub guid: String,
    pub unread: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feed {
    pub title: String,
    pub last_updated: String,
    pub items: Vec<Item>,
}

#[wasm_bindgen]
pub fn update_feed(url: String) -> Promise {
    let window = web_sys::window().unwrap();
    let storage = if let Ok(Some(storage)) = window.local_storage() { storage } else {
        return future_to_promise(future::err("No local storage available".into()));
    };
    let future = JsFuture::from(window.fetch_with_str(&url))
        .map(move |response| (response, url, storage))
        .and_then(|(response, url, storage)| {
            assert!(response.is_instance_of::<Response>());
            future::result(
                response.dyn_into::<Response>()
                    .map_err(|_| "Invalid response received".into())
                    .and_then(|response| response.text())
                    .map(move |text| (text, url, storage))
            )
        })
        .and_then(|(text, url, storage)| {
            let text: String = text.to_string().into();
            future::result(
                rss::Channel::read_from(text.as_bytes())
                    .map_err(|e| e.to_string().into())
                    .map(move |channel| (channel, url, storage))
            )
        })
        .and_then(|(channel, url, storage)| future::result::<(Feed, String, Storage), JsValue>(
            storage
                .get_item(&url)
                .map(|str_opt| str_opt.unwrap_or("".into()))
                .and_then(|s| Ok(jsons::from_str(&s).unwrap_or(Feed{title: "".to_owned(), last_updated: "".to_owned(), items: vec![]})))
                .and_then(move |mut feed| {
                    feed.title = channel.title().to_owned();
                    feed.last_updated = channel.last_build_date().ok_or(JsValue::from_str("Incorrect feed"))?.to_owned();
                    Ok((feed, url, storage))
                })
        ))
        .and_then(|(mut feed, url, storage)| {
            let mut items: HashMap<&str, Item> = feed.items.iter().map(|item| (item.guid.as_str(), item.clone())).collect();
            for item in feed.items.iter() {
                let guid = &item.guid;
                items.entry(&guid).or_insert_with(|| Item {
                    title: item.title.to_owned(),
                    link: item.link.to_owned(),
                    content: item.content.to_owned(),
                    pub_date: item.pub_date.to_owned(),
                    guid: guid.to_owned(),
                    unread: true,
                });
            }
            feed.items = items.drain().map(|(_, v)| v).collect();
            feed.items.sort_by_key(|item| item.pub_date.clone());
            future::result(storage.set_item(&url.clone(), &jsons::to_string(&feed).unwrap()).map(|_| JsValue::null()))
        });

    wasm_bindgen_futures::future_to_promise(future)
}
