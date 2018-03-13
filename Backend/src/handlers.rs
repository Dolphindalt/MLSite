use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use iron::headers::ContentType;
use rustc_serialize::json;
use database::Database;
use uuid::Uuid;
use router::Router;
use std::error::Error;

use models::NewsPost;
use database::NEWS_POST_COLLECTION;

macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.description())))
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.description())))
        }
    }
}

macro_rules! lock {
    ($e:expr) => {$e.lock().unwrap()}
}

macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get::<Router>() {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with(status::BadRequest)),
                }
            },
            None => return Ok(Response::with(status::InternalServerError))
        }
    }
}

pub struct Handlers {
    pub news_post_handler: NewsPostHandler,
    pub news_post_feed_handler: NewsPostFeedHandler,
    pub news_post_post_handler: NewsPostPostHandler,
}

impl Handlers {
    pub fn new(database: Database) -> Handlers {
        let db = Arc::new(Mutex::new(database));
        Handlers {
            news_post_handler: NewsPostHandler::new(db.clone()),
            news_post_post_handler: NewsPostPostHandler::new(db.clone()),
            news_post_feed_handler: NewsPostFeedHandler::new(db.clone()),
        }
    }
}

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
    fn new(database: Arc<Mutex<Database>>) -> NewsPostPostHandler {
        NewsPostPostHandler { database }
    }
}

impl Handler for NewsPostPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let news_post = try_handler!(json::decode(&payload), status::BadRequest);

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
        if let Some(news_post) = locked.find_document::<NewsPost>(NEWS_POST_COLLECTION, &id) {
            let payload = try_handler!(json::encode(&news_post), status::InternalServerError);
            Ok(Response::with((status::Ok, payload)))
        } else {
            Ok(Response::with(status::NotFound))
        }
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}