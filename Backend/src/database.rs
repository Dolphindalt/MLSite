use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson;
use uuid::Uuid;
use serde::de::Deserialize;

use models::NewsPost;
use models::User;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 27017;
const DB: &str = "test";

pub const USER_COLLECTION: &str = "users";
pub const NEWS_POST_COLLECTION: &str = "newsposts";

pub struct Database {
    client: Client
}

impl Database {
    /// Constructs a new Database struct for handling all mongo related functions.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use database::Database;
    /// 
    /// let database = Database::new();
    /// ```
    pub fn new() -> Database {
        Database { client: Client::connect(HOSTNAME, PORT)
            .expect("Failed to connect to the mongo database") }
    }

    /// Adds a new user ti the user collection in the mongo database.
    pub fn add_user(&mut self, user: User) {
        let collection = self.client.db(DB).collection(USER_COLLECTION);

        let doc = doc! {
            "username": user.username,
            "hashword": user.hashword,
            "date_created": user.date_created,
        };

        collection.insert_one(doc.clone(), None)
            .ok().expect("Failed to insert the new user!");
   }

    /// Adds a news posts to the news posts collection in the mongo database.
    pub fn add_news_post(&mut self, news_post: NewsPost) {
        let collection = self.client.db(DB).collection(NEWS_POST_COLLECTION);

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

    /// Get all documents from a specified collection.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use database::Database;
    /// use database::NEWS_POST_COLLECTION;
    /// use models::NewsPost;
    /// 
    /// let database = Database::new();
    /// let vec_of_newsposts = database.get_all_documents::<NewsPost>(NEWS_POST_COLLECTION);
    /// ```
    pub fn get_all_documents<T>(&self, collection: &str) -> Vec<T> 
        where T: Deserialize<'static> {
        let mut docs = Vec::new();
        let collection = self.client.db(DB).collection(collection);
        let cursor = collection.find(None, None).ok().expect("Failed to execute find");
        
        for doc in cursor {
            let d = doc.unwrap();
            docs.push(
                bson::from_bson::<T>(bson::Bson::Document(d)).unwrap()
            );
        };
        docs
    }

    /// Finds a document in the given collection with the given uuid.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use database::Database;
    /// use database::NEWS_POST_COLLECTION;
    /// use models::NewsPost;
    /// use uuid::Uuid;
    /// 
    /// let database = Database::new();
    /// let ret = database.find_document_with_uuid::<NewsPost>(NEWS_POST_COLLECTION, Uuid::new_v4());
    /// 
    /// match ret {
    ///     Some(thing) => println!("We found a document!"),
    ///     None => println!("We did not find a document!"),
    /// };
    /// ```
    pub fn find_document_with_uuid<T>(&self, collection: &str, id: &Uuid) -> Option<T>
        where T: Deserialize<'static> {
        let collection = self.client.db(DB).collection(collection);
        let result = collection.find_one(Some(doc!{ "uuid" => id.to_string() }), None);

        if let Some(doc) = result.unwrap() {
            Some(bson::from_bson::<T>(bson::Bson::Document(doc)).unwrap())
        } else {
            None
        }
    }

    pub fn find_document_with_username<T>(&self, collection: &str, username: &String) -> Option<T> 
        where T: Deserialize<'static> {
        let collection = self.client.db(DB).collection(collection);
        let result = collection.find_one(Some(doc!{ "username" => username}), None);

        if let Some(doc) = result.unwrap() {
            Some(bson::from_bson::<T>(bson::Bson::Document(doc)).unwrap())
        } else {
            None
        }
    }

    /// Erase all documents from the given collection matching the query provided.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use database::Database;
    /// use database::NEWS_POST_COLLECTION;
    /// use bson;
    /// 
    /// let database = Database::new();
    /// database.erase_from_collection_where(NEWS_POST_COLLECTION, doc!{});
    /// ```
    pub fn erase_from_collection_where(&self, collection: &str, query: bson::Document) {
        let collection = self.client.db(DB).collection(collection);
        match collection.delete_many(query, None) {
            Ok(_delete_result) => (),
            Err(err) => panic!("{:?}", err),
        }
    }
}

#[test]
fn news_posts_test() {
    let mut connection = Database::new();

    connection.erase_from_collection_where(NEWS_POST_COLLECTION, doc!{});

    let posts = connection.get_all_documents::<NewsPost>(NEWS_POST_COLLECTION);

    assert!(posts.len() == 0);

    let p1 = NewsPost {
        title: "The first post".to_string(),
        body: "Behold, here is the first post".to_string(),
        author: "Dalton Caron".to_string(),
        datetime: "03/10/18".to_string(),
        uuid: Uuid::new_v4().to_string(),
    };

    connection.add_news_post(p1);

    let posts = connection.get_all_documents::<NewsPost>(NEWS_POST_COLLECTION);

    assert!(posts.len() == 1);

    connection.erase_from_collection_where(NEWS_POST_COLLECTION, doc!{});

    let posts = connection.get_all_documents::<NewsPost>(NEWS_POST_COLLECTION);

    assert!(posts.len() == 0);
}

#[test]
fn insert_news_posts_test() {
    let mut connection = Database::new();

    let p1 = NewsPost {
        title: "The first post".to_string(),
        body: "Behold, here is the first post".to_string(),
        author: "Dalton Caron".to_string(),
        datetime: "03/10/18".to_string(),
        uuid: Uuid::new_v4().to_string(),
    };

    let p2 = NewsPost {
        title: "The second post".to_string(),
        body: "Behold, here is the second post".to_string(),
        author: "Dalton Caron".to_string(),
        datetime: "03/10/18".to_string(),
        uuid: Uuid::new_v4().to_string(),
    };

    connection.add_news_post(p1);
    connection.add_news_post(p2);
}