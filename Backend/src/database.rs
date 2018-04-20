use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use bson;
use uuid::Uuid;
use serde::de::Deserialize;
use mongodb::coll::options::{FindOptions, UpdateOptions};

use models::NewsPost;
use models::User;
use models::Email;
use models::ForumPost;

use helpers;

const HOSTNAME: &str = "localhost";
const PORT: u16 = 27017;
const DB: &str = "test";

pub const USER_COLLECTION: &str = "users";
pub const NEWS_POST_COLLECTION: &str = "newsposts";
pub const EMAIL_REQUEST_COLLECTION: &str = "emails";

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

    /// Adds a new user to the user collection in the mongo database.
    pub fn add_user(&mut self, user: User) {
        let collection = self.client.db(DB).collection(USER_COLLECTION);

        let doc = doc! {
            "hashword": user.hashword,
            "email": user.email,
            "admin": user.admin,
            "date_created": user.date_created,
            "uuid": user.uuid,
            "staff": user.staff,
            "rank": user.rank,
        };

        collection.insert_one(doc.clone(), None)
            .ok().expect("Failed to insert the new user!");
    }

    /// Adds a new email request to the emails collection in the mongo database.
    pub fn add_email_request(&mut self, email: Email) {
        let collection = self.client.db(DB).collection(EMAIL_REQUEST_COLLECTION);

        let doc = doc! {
            "uuid": email.uuid,
            "email": email.email,
            "linkUuid": email.linkUuid,
        };

        collection.insert_one(doc.clone(), None)
            .ok().expect("Failed to insert email request");
    }

    pub fn add_forum_post(&mut self, category: &str, post_chain: ForumPost) {
        let collection = self.client.db(DB).collection(category);

        let doc = helpers::encode::<ForumPost>(&post_chain).unwrap();

        collection.insert_one(doc, None)
            .ok().expect("Failed to insert a forum post chain");
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
    pub fn get_all_documents<T>(&self, collection: &str, doc_opt: Option<bson::Document>, find_opt: Option<FindOptions>) -> Vec<T> 
        where T: Deserialize<'static> {
        let mut docs = Vec::new();
        let collection = self.client.db(DB).collection(collection);
        let cursor = collection.find(doc_opt, find_opt).ok().expect("Failed to execute find");
        
        for doc in cursor {
            let d = doc.unwrap();
            docs.push(
                bson::from_bson::<T>(bson::Bson::Document(d)).unwrap()
            );
        };
        docs
    }

    /// Find a single document.
    pub fn find_one_document<T>(&self, collection: &str, doc_opt: Option<bson::Document>, find_opt: Option<FindOptions>) -> Option<T> 
        where T: Deserialize<'static> {
            let collection = self.client.db(DB).collection(collection);
            if let Ok(t) = collection.find_one(doc_opt, find_opt) {
                if let Some(v) = t {
                    if let Ok(doc) = bson::from_bson::<T>(bson::Bson::Document(v)) {
                        Some(doc)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
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
    #[deprecated]
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

    /// Updates a single document in the mongo collection provided.
    pub fn update_document<T>(&self, collection: &str, filter: bson::Document, update: bson::Document, options: Option<UpdateOptions>) -> bool {
        let collection = self.client.db(DB).collection(collection);
        if let Ok(_) = collection.update_one(filter, update, options) {
            true
        } else {
            false
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