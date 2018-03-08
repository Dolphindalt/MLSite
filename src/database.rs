use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson;

use models::NewsPost;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 27017;
const DB: &str = "test";

struct Database {
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
        let mut cursor = collection.find(None, None).ok().expect("Failed to execute find");
        
        for doc in cursor {
            let d = doc.unwrap();
            news_posts.push(
                bson::from_bson::<NewsPost>(bson::Bson::Document(d)).unwrap()
            );
        };
        news_posts
    }
}