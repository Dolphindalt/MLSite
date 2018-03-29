#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;
extern crate uuid;
extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate unicase;
extern crate jsonwebtoken as jwt;

mod database;
mod models;
#[macro_use]
mod handlers;
mod login_handlers;
mod news_post_handlers;

use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;

fn main() {
    let database = Database::new();

    let handlers = Handlers::new(database);
    let json_content_middleware = JsonAfterMiddleware;
    let cors_after_middleware = CorsAfterMiddleWare;

    let mut router = Router::new();
    router.get("/home", handlers.news_post_feed_handler, "home");
    router.get("/home_post", handlers.news_post_post_handler, "home_newspost");
    router.get("/home_post/:id", handlers.news_post_handler, "home_newspost_id");
    router.post("/register", handlers.user_created_handler, "user_created");
    router.post("/login", handlers.login_request_handler, "user_login");

    let mut chain = Chain::new(router);
    chain.link_after(json_content_middleware);
    chain.link_after(cors_after_middleware);

    Iron::new(chain).http("localhost:8000").unwrap();
}
