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
extern crate lettre_email;
extern crate lettre;

mod database;
mod models;
#[macro_use]
mod handlers;
mod user_handlers;
mod news_post_handlers;
mod userdata;
mod forum_handlers;
mod helpers;

use database::Database;
use handlers::*;

use iron::prelude::Chain;
use iron::Iron;
use router::Router;

const IP_AND_HOST: &str = "127.0.0.1:8000";

fn main() {
    let database = Database::new();

    let handlers = Handlers::new(database);
    let json_content_middleware = JsonAfterMiddleware;
    let cors_after_middleware = CorsAfterMiddleWare;

    let mut router = Router::new();
    router.get("/admin", handlers.admin_handler, "admin");

    router.get("/news-post-models", handlers.news_post_feed_handler, "home");
    router.post("/news-post-models", handlers.news_post_post_handler, "home_newspost");
    router.get("/news-post-models/:id", handlers.news_post_handler, "home_newspost_id");

    router.post("/register", handlers.user_created_handler, "user_pending");
    router.get("/register/:linkUuid", handlers.get_user_register_handler, "prompting_user_password");
    router.post("/register/:linkUuid", handlers.user_register_handler, "user_registered");
    router.post("/login", handlers.login_request_handler, "user_login");

    router.get("/user/:uuid", handlers.user_get_single_handler, "user_get_single");
    router.get("/user/staff", handlers.user_get_staff_handler, "user_get_staff");
    
    router.get("/forums/:category", handlers.get_all_posts_handler, "get_all_posts");
    router.post("/forums/create/:category", handlers.create_post_handler, "create_post");
    router.get("/forums/stats/category/:category", handlers.get_category_stats_and_last_post, "get_category_stats");
    router.get("/forums/:category/:page", handlers.get_forum_listing_data, "get_forum_listings");

    router.post("/forums/newthread/:category", handlers.post_thread_to_forum, "create_new_thread");
    router.post("/forums/newpost/:category/:thread_uuid", handlers.post_post_to_thread, "create_new_post");
    
    router.get("/search/:term", handlers.search_users_handler, "search_users");

    let mut chain = Chain::new(router);
    chain.link_after(json_content_middleware);
    chain.link_after(cors_after_middleware);

    println!("Now serving the site on {}", IP_AND_HOST);
    Iron::new(chain).http("127.0.0.1:8000").unwrap();
}
