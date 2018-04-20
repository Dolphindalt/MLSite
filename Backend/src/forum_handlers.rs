use std::sync::{Arc, Mutex};
use std::io::Read;
use iron::{status, Handler, IronResult, Request, Response};
use rustc_serialize::json;
use database::Database;
use std::error::Error;
use jwt::{encode, Header};
use serde_json;
use serde_json::Value;
use router::Router;
use bson;

use models::ForumPost;
use models::Post;

use helpers;

pub struct GetAllPostsHandler {
    database: Arc<Mutex<Database>>,
}

impl GetAllPostsHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> GetAllPostsHandler {
        GetAllPostsHandler { database }
    }
}

impl Handler for GetAllPostsHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");

        let forum_posts = lock!(self.database).get_all_documents::<ForumPost>(category, None, None);
        let payload = try_handler!(json::encode(&forum_posts), status::BadRequest);

        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct CreatePostHandler {
    database: Arc<Mutex<Database>>,
}

impl CreatePostHandler {
    pub fn new(database: Arc<Mutex<Database>>) -> CreatePostHandler {
        CreatePostHandler { database }
    }
}

impl Handler for CreatePostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");

        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let the_new_post: Post;
        if let Ok(post) = serde_json::from_str::<Post>(&payload) {
            the_new_post = post;
        } else {
            return Ok(Response::with(status::BadRequest))
        };

        // Does a post chain with this uuid already exist?
        let post_chain = lock!(self.database).find_one_document::<ForumPost>(category, Some(doc!{ "chain_uuid" : &the_new_post.uuid }), None);

        if let Some(mut chain) = post_chain {
            chain.posts.push(the_new_post);
            let doc_update: bson::Document = helpers::encode::<ForumPost>(&chain).unwrap();
            if lock!(self.database).update_document::<ForumPost>(category, doc!{ "chain_uuid" => &chain.chain_uuid}, doc_update, None) {
                Ok(Response::with(status::Ok))
            } else {
                Ok(Response::with(status::NotFound))
            }
        } else {
            let uuid = the_new_post.uuid.clone();
            let new_chain = ForumPost::new(uuid, the_new_post);
            lock!(self.database).add_forum_post(category, new_chain);
            Ok(Response::with(status::Ok))
        }
    }
}