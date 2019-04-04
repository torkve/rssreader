mod utils;

use chrono::{DateTime, Utc, TimeZone};
use actix::{Actor, Handler, Message, AsyncContext};
use actix_web::{http, web, HttpResponse, App};
use actix_web::middleware::cors::Cors;
use futures::{Future};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::debug;

use crate::utils::ErrString;

#[derive(Debug, Serialize, Clone)]
struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub content: Option<String>,
    pub pub_date: Option<DateTime<Utc>>,
    pub guid: String,
    pub unread: bool,
}

#[derive(Clone, Debug, Serialize)]
struct Feed {
    pub title: String,
    pub last_updated: DateTime<Utc>,
    pub items: Vec<Item>,
}
impl Feed {
    pub fn merge(&mut self, other: Feed) {
        self.title = other.title;
        self.last_updated = other.last_updated;

        let mut items: HashMap<&str, Item> = self.items.iter().map(|item| (item.guid.as_str(), item.clone())).collect();
        for item in other.items.iter() {
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
        self.items = items.drain().map(|(_, v)| v).collect();
        self.items.sort_by_key(|item| item.pub_date.clone());
    }
}

#[derive(Serialize)]
struct FeedInfo {
    pub url: String,
    pub title: String,
    pub last_updated: DateTime<Utc>,
}

struct DownloadFeed(String);
#[derive(Deserialize)]
struct AddFeed { url: String }
#[derive(Deserialize)]
struct RemoveFeed { url: String }
#[derive(Deserialize, Debug)]
struct GetFeed { url: String }
struct ListFeeds;
#[derive(Deserialize, Debug)]
struct MarkRead { url: String, guid: String }
#[derive(Message)]
struct UpdateFeed { url: String, feed: Feed }

impl Message for DownloadFeed {
    type Result = Result<Feed, String>;
}
impl Message for AddFeed {
    type Result = Result<(), String>;
}
impl Message for RemoveFeed {
    type Result = Result<(), String>;
}
impl Message for GetFeed {
    type Result = Result<Feed, String>;
}
impl Message for ListFeeds {
    type Result = Result<Vec<FeedInfo>, String>;
}
impl Message for MarkRead {
    type Result = Result<bool, String>;
}
struct FeedStorage {
    feeds: HashMap<String, Feed>,
    downloader: actix::Addr<Downloader>,
}
impl Actor for FeedStorage {
    type Context = actix::SyncContext<Self>;
}
impl Handler<DownloadFeed> for FeedStorage {
    type Result = <DownloadFeed as Message>::Result;

    fn handle(&mut self, msg: DownloadFeed, _: &mut Self::Context) -> Self::Result {
        self.downloader.send(msg).wait().or_err("Download failed")?
    }
}
impl Handler<AddFeed> for FeedStorage {
    type Result = <AddFeed as Message>::Result;

    fn handle(&mut self, msg: AddFeed, _: &mut Self::Context) -> Self::Result {
        match self.feeds.entry(msg.url.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => Err("Feed already exists".into()),
            std::collections::hash_map::Entry::Vacant(e) => {
                debug!("will download {}", &msg.url);
                self.downloader.send(DownloadFeed(msg.url))
                    .wait()
                    .or_err("Failed to download")?
                    .map(|feed| {
                        debug!("downloaded");
                        e.insert(feed);
                    })
            }
        }
    }
}
impl Handler<RemoveFeed> for FeedStorage {
    type Result = <RemoveFeed as Message>::Result;

    fn handle(&mut self, msg: RemoveFeed, _: &mut Self::Context) -> Self::Result {
        self.feeds.remove(&msg.url);
        Ok(())
    }
}
impl Handler<GetFeed> for FeedStorage {
    type Result = <GetFeed as Message>::Result;

    fn handle(&mut self, msg: GetFeed, _: &mut Self::Context) -> Self::Result {
        match self.feeds.get(&msg.url) {
            None => Err("Feed not found".into()),
            Some(feed) => Ok(feed.clone()),
        }
    }
}
impl Handler<ListFeeds> for FeedStorage {
    type Result = <ListFeeds as Message>::Result;

    fn handle(&mut self, _: ListFeeds, _: &mut Self::Context) -> Self::Result {
        Ok(self.feeds.iter().map(|(k, v)| FeedInfo{url: k.clone(), title: v.title.clone(), last_updated: v.last_updated.clone()}).collect())
    }
}
impl Handler<MarkRead> for FeedStorage {
    type Result = <MarkRead as Message>::Result;

    fn handle(&mut self, msg: MarkRead, _: &mut Self::Context) -> Self::Result {
        let mut updated = false;
        if let Some(feed) = self.feeds.get_mut(&msg.url) {
            for item in feed.items.iter_mut().filter(|k| &k.guid == &msg.guid).take(1) {
                item.unread = false;
                updated = true;
            }
        }
        Ok(updated)
    }
}
impl Handler<UpdateFeed> for FeedStorage {
    type Result = <UpdateFeed as Message>::Result;
    fn handle(&mut self, msg: UpdateFeed, _: &mut Self::Context) -> Self::Result {
        if let Some(feed) = self.feeds.get_mut(&msg.url) {
            feed.merge(msg.feed)
        };
    }
}


struct Downloader;
impl Actor for Downloader {
    type Context = actix::Context<Self>;
}
impl Handler<DownloadFeed> for Downloader {
    type Result = <DownloadFeed as Message>::Result;

    fn handle(&mut self, msg: DownloadFeed, _: &mut Self::Context) -> Self::Result {
        let channel = rss::Channel::from_url(&msg.0).or_err("Channel not downloaded")?;
        let mut items = vec![];
        for item in channel.items().iter() {
            let guid = item.guid().or_err("broken channel")?.value();
            items.push(Item {
                title: item.title().map(|s| s.to_string()),
                link: item.link().map(|s| s.to_string()),
                content: item.content().or(item.description()).map(|s| s.to_string()),
                pub_date: item.pub_date().and_then(|date| DateTime::parse_from_rfc2822(date).ok().map(|d| d.with_timezone(&Utc))),
                guid: guid.to_string(),
                unread: true,
            });
        }
        Ok(Feed{
            title: channel.title().to_owned(),
            last_updated: match channel.last_build_date() {
                None => items
                    .iter()
                    .map(|item| &item.pub_date)
                    .max()
                    .map(|date| date.to_owned())
                    .unwrap_or(Some(Utc.timestamp(0, 0)))
                    .unwrap_or(Utc.timestamp(0, 0)),
                Some(s) => DateTime::parse_from_rfc2822(s).map(|d| d.with_timezone(&Utc)).unwrap_or(Utc.timestamp(0, 0))
            },
            items: items
        })
    }
}

struct Updater {
    storage: actix::Addr<FeedStorage>,
    downloader: actix::Addr<Downloader>,
    handle: Option<actix::SpawnHandle>,
    arbiter: actix::Arbiter,
}
impl Actor for Updater {
    type Context = actix::Context<Self>;
    fn started(&mut self, ctx: &mut <Self as Actor>::Context) {
        let storage = self.storage.clone();
        let downloader = self.downloader.clone();
        let arbiter = self.arbiter.clone();
        self.handle = Some(ctx.run_interval(std::time::Duration::new(60, 0), move |_, _| {
            let storage = storage.clone();
            let downloader = downloader.clone();
            let arbiter = arbiter.clone();
            arbiter.exec_fn(move || {
                if let Ok(Ok(infos)) = storage.send(ListFeeds).wait() {
                    debug!("got {} feeds, updating", infos.len());
                    for info in infos {
                        if let Ok(Ok(new_feed)) = downloader.send(DownloadFeed(info.url.clone())).wait() {
                            if let Ok(()) = storage.send(UpdateFeed{url: info.url.clone(), feed: new_feed}).wait() {
                                debug!("successfully updated {}", info.url);
                            }
                        }
                    }
                }
            });
        }));
    }
}

fn process_response<T: Serialize, E: Serialize, E2, F: FnOnce(E) -> actix_web::Error>(response: Result<Result<T, E>, E2>, f: F) -> Result<HttpResponse, actix_web::Error> {
    match response {
        Ok(Ok(data)) => Ok(HttpResponse::Ok().json(data)),
        Ok(Err(e)) => Err(f(e)),
        _ => Err(actix_web::error::ErrorInternalServerError("Application overload"))
    }
}

#[derive(Clone)]
struct State { storage: actix::Addr<FeedStorage> }
fn add_feed(url_info: web::Form<AddFeed>, data: web::Data<State>) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || data.storage.send(url_info.into_inner()).wait())
        .then(|res| process_response(res, actix_web::error::ErrorInternalServerError))
}
fn remove_feed(url_info: web::Form<RemoveFeed>, data: web::Data<State>) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || data.storage.send(url_info.into_inner()).wait())
        .then(|res| process_response(res, actix_web::error::ErrorInternalServerError))
}
fn get_feed(url_info: web::Query<GetFeed>, data: web::Data<State>) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || data.storage.send(url_info.into_inner()).wait())
        .then(|res| process_response(res, actix_web::error::ErrorNotFound))
}
fn list_feeds(data: web::Data<State>) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || data.storage.send(ListFeeds).wait())
        .then(|res| process_response(res, actix_web::error::ErrorInternalServerError))
}
fn mark_read(url_info: web::Form<MarkRead>, data: web::Data<State>) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    web::block(move || data.storage.send(url_info.into_inner()).wait())
        .then(|res| process_response(res, actix_web::error::ErrorInternalServerError))
}

fn actix_main() -> Result<(), std::io::Error> {
    let downloader_addr = Downloader.start();
    let feed_storage_addr = {
        let addr = downloader_addr.clone();
        actix::SyncArbiter::start(1, move || FeedStorage{
            feeds: HashMap::new(),
            downloader: addr.clone(),
        })
    };
    let state = State{storage: feed_storage_addr.clone()};

    let updater = Updater{storage: feed_storage_addr, downloader: downloader_addr.clone(), handle: None, arbiter: actix::Arbiter::new()};
    updater.start();

    let mut server = actix_web::HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(
                Cors::new()
                .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                .allowed_headers(vec![
                                 http::header::ACCEPT,
                                 http::header::CONTENT_TYPE,
                                 http::header::HeaderName::from_static("x-requested-with")
                ])
                .max_age(3600)
            )
            .wrap(actix_web::middleware::Logger::default())
            .route("/add", web::post().to_async(add_feed))
            .route("/remove", web::post().to_async(remove_feed))
            .route("/read", web::post().to_async(mark_read))
            .route("/list", web::get().to_async(list_feeds))
            .route("/get", web::get().to_async(get_feed))
    });

    let mut listenfd = listenfd::ListenFd::from_env();
    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind("[::1]:8000")?
    };
    println!("Started HTTP server on {:?}", server.addrs_with_scheme().iter().map(|(a, s)| format!("{}://{}/", s, a)).collect::<Vec<_>>());
    server.start();
    Ok(())
}

pub fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=debug,rssreader=debug");
    env_logger::init();
    actix::System::run(|| {actix_main().expect("App crashed");} ) 
}
