use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::results::DeleteResult;
use mongodb::error::Error;
use bson;
use uuid::Uuid;

use models::NewsPost;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 27017;
const DB: &str = "test";

pub struct Database {
    client: Client
}

impl Database {
    pub fn new() -> Database {
        Database { client: Client::connect(HOSTNAME, PORT)
            .expect("Failed to connect to the mongo database") }
    }

    pub fn add_news_post(&mut self, news_post: NewsPost) {
        let collection = self.client.db(DB).collection("newsposts");

        let doc = doc! {
            "title": news_post.title,
            "body": news_post.body,
            "author": news_post.author,
            "datetime": news_post.datetime,
            "uuid": news_post.uuid,
        };

        collection.insert_one(doc.clone(), None)
            .ok().expect("Failed to insert News Post");
    }

    pub fn get_news_posts(&self) -> Vec<NewsPost> {
        let mut news_posts = Vec::new();
        let collection = self.client.db(DB).collection("newsposts");
        let cursor = collection.find(None, None).ok().expect("Failed to execute find");
        
        for doc in cursor {
            let d = doc.unwrap();
            news_posts.push(
                bson::from_bson::<NewsPost>(bson::Bson::Document(d)).unwrap()
            );
        };
        news_posts
    }

    pub fn find_news_post(&self, id: &Uuid) -> Option<NewsPost> {
        let collection = self.client.db(DB).collection("newsposts");
        let result = collection.find_one(Some(doc!{ "uuid" => id.to_string() }), None);

        if let Some(doc) = result.unwrap() {
            Some(bson::from_bson::<NewsPost>(bson::Bson::Document(doc)).unwrap())
        } else {
            None
        }
    }

    pub fn erase_collection_where(&self, cstr: &str, query: bson::Document) {
        let collection = self.client.db(DB).collection(cstr);
        match collection.delete_many(query, None) {
            Ok(_delete_result) => (),
            Err(err) => panic!("{:?}", err),
        }
    }
}

#[test]
fn news_posts_test() {
    let mut connection = Database::new();

    let posts = connection.get_news_posts();

    assert!(posts.len() == 0);

    let p1 = NewsPost {
        title: "The first post".to_string(),
        body: "Behold, here is the first post".to_string(),
        author: "Dalton Caron".to_string(),
        datetime: "03/10/18".to_string(),
        uuid: Uuid::new_v4().to_string(),
    };

    connection.add_news_post(p1);

    let posts = connection.get_news_posts();

    assert!(posts.len() == 1);

    connection.erase_collection_where("newsposts", doc!{});

    let posts = connection.get_news_posts();

    assert!(posts.len() == 0);
}