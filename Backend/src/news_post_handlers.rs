use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use database::Database;
use uuid::Uuid;
use router::Router;
use std::error::Error;
use serde_json;

use models::NewsPost;
use database::NEWS_POST_COLLECTION;

pub struct NewsPostFeedHandler {
    database: Arc<Mutex<Database>>
}

impl NewsPostFeedHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> NewsPostFeedHandler {
        NewsPostFeedHandler { database }
    }
}

impl Handler for NewsPostFeedHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = try_handler!(json::encode(&lock!(self.database).get_all_documents::<NewsPost>(NEWS_POST_COLLECTION)));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct NewsPostPostHandler {
    database: Arc<Mutex<Database>>
}

impl NewsPostPostHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> NewsPostPostHandler {
        NewsPostPostHandler { database }
    }
}

impl Handler for NewsPostPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let news_post: NewsPost = serde_json::from_str::<NewsPost>(&payload).unwrap(); // fix this later
        
        lock!(self.database).add_news_post(news_post);
        Ok(Response::with((status::Created, payload)))
    }
}

pub struct NewsPostHandler {
    database: Arc<Mutex<Database>>
}

impl NewsPostHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> NewsPostHandler {
        NewsPostHandler { database }
    }
}

impl Handler for NewsPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref post_id = get_http_param!(req, "id");

        let id = try_handler!(Uuid::parse_str(post_id), status::BadRequest);

        let locked = lock!(self.database);
        if let Some(news_post) = locked.find_document_with_uuid::<NewsPost>(NEWS_POST_COLLECTION, &id) {
            let payload = try_handler!(json::encode(&news_post), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}