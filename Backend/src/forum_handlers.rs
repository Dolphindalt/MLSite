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
use mongodb::coll::options::{FindOptions};

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

pub struct GetCategoryStatsAndLastPost {
    database: Arc<Mutex<Database>>
}

impl GetCategoryStatsAndLastPost {
    pub fn new(database: Arc<Mutex<Database>>) -> GetCategoryStatsAndLastPost {
        GetCategoryStatsAndLastPost { database }
    }
}

impl Handler for GetCategoryStatsAndLastPost {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");
        let forum_posts = lock!(self.database).get_all_documents::<ForumPost>(category, None, None);
        let total_threads = forum_posts.len();
        let mut total_posts = 0;
        
        for forum_post in &forum_posts {
            total_posts = total_posts + forum_post.posts.len();
        }

        let mut fo = FindOptions::new();
        fo.sort = Some(doc!{"_id" => -1});

        let last_post = lock!(self.database).find_one_document::<ForumPost>(category, None, Some(fo));
        let (title, author, last_thread_by_uuid, last_thread_uuid);
        if let Some(lp) = last_post {
            title = lp.posts[0].title.clone();
            author = lp.posts[0].author.clone();
            last_thread_by_uuid = lp.posts[0].author_uuid.clone();
            last_thread_uuid = lp.chain_uuid.clone();
        } else {
            title = String::from("There are no posts");
            author = String::from("");
            last_thread_by_uuid = String::from("");
            last_thread_uuid = String::from("");
        };

        let payload = format!("{{ \"threads\" : \"{}\", \"posts\" : \"{}\", \"last_thread\" : \"{}\", \"last_thread_uuid\" : \"{}\", \"last_thread_by\" : \"{}\", \"last_thread_by_uuid\" : \"{}\"}}", total_threads, total_posts, title, last_thread_uuid, author, last_thread_by_uuid);

        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct GetForumListingData {
    database: Arc<Mutex<Database>>
}

impl GetForumListingData {
    pub fn new(database: Arc<Mutex<Database>>) -> GetForumListingData {
        GetForumListingData { database }
    }
}

const POSTINCR: usize = 10;

impl Handler for GetForumListingData {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");
        let mut page = try_handler!(get_http_param!(req, "page").to_string().parse::<usize>(), status::BadRequest);

        let forum_posts = lock!(self.database).get_all_documents::<ForumPost>(category, None, None);

        let total_threads = &forum_posts.len();
        let max_page = (total_threads / POSTINCR) + match total_threads % 10 {
            0 => 0,
            _ => 1,
        };

        if max_page == 0 {
            return Ok(Response::with((status::Ok, "[]")));
        }

        if page <= 0 {
            page = 1;
        } else if page > max_page {
            page = max_page;
        }

        page = page - 1;

        let mut start = page * POSTINCR;
        let end = page * POSTINCR + POSTINCR;
        let mut post_data = Vec::new();

        loop {
            if &start >= total_threads || start == end {
                break;
            }

            let mut temp = forum_posts[start].posts[0].clone();
            let last_poster = forum_posts[start].posts[forum_posts[start].posts.len()-1].author.clone();
            temp.uuid = forum_posts[start].chain_uuid.clone();
            post_data.push(temp.convert(forum_posts[start].posts.len(), last_poster));
            start = start + 1;
        }

        let payload = try_handler!(json::encode(&post_data), status::BadRequest);

        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct PostThreadToForum {
    database: Arc<Mutex<Database>>,
}

impl PostThreadToForum {
    pub fn new(database: Arc<Mutex<Database>>) -> PostThreadToForum {
        PostThreadToForum { database }
    }
}

impl Handler for PostThreadToForum {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");

        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let post: ForumPost;
        if let Ok(some_data) = serde_json::from_str::<ForumPost>(&payload) {
            post = some_data;
        } else {
            return Ok(Response::with((status::BadRequest, "Failed to parse post data!")))
        };

        lock!(self.database).add_forum_post(category, post);

        Ok(Response::with(status::Ok))
    }
}

pub struct PostPostToThread {
    database: Arc<Mutex<Database>>,
}

impl PostPostToThread {
    pub fn new(database: Arc<Mutex<Database>>) -> PostPostToThread {
        PostPostToThread { database }
    }
}

impl Handler for PostPostToThread {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");
        let uuid = get_http_param!(req, "thread_uuid");

        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));

        let post: Post;
        if let Ok(some_data) = serde_json::from_str::<Post>(&payload) {
            post = some_data;
        } else {
            return Ok(Response::with((status::BadRequest, "Failed to parse post data!")))
        };

        let opt = lock!(self.database).find_one_document::<ForumPost>(category, Some(doc!{"chain_uuid" => uuid}), None);

        if let Some(mut forum_post) = opt {
            forum_post.posts.push(post);

            if let bson::Bson::Document(document) = bson::to_bson(&forum_post).unwrap() {
                lock!(self.database).replace_one_document(category, doc!{"chain_uuid" => uuid}, document);
                Ok(Response::with((status::Ok, "{}")))
            } else {
                Ok(Response::with((status::Conflict, "Decodeder error!")))
            }
        } else {
            Ok(Response::with((status::NotFound, "The thread was lost!")))
        }
    }
}

pub struct GetForumThread {
    database: Arc<Mutex<Database>>,
}

impl GetForumThread {
    pub fn new(database: Arc<Mutex<Database>>) -> GetForumThread {
        GetForumThread { database }
    }
}

impl Handler for GetForumThread {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let category = get_http_param!(req, "category");
        let uuid = get_http_param!(req, "thread_uuid");

        if let Some(result) = lock!(self.database).find_one_document::<ForumPost>(category, Some(doc!{"chain_uuid" => uuid}), None) {
            Ok(Response::with((status::Ok, try_handler!(json::encode(&result)))))
        } else {
            Ok(Response::with((status::NotFound, "The thread was lost!")))
        }
    }
}