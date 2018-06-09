#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct NewsPost {
    pub title: String,
    pub body: String,
    pub author: String,
    pub datetime: String,
    pub uuid: String,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct User {
    pub hashword: String,
    pub username: String,
    pub email: String,
    pub admin: bool,
    pub date_created: String,
    pub uuid: String,
    pub staff: bool,
    pub rank: String,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct Email {
    pub uuid: String,
    pub email: String,
    pub linkUuid: String,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub author: String,
    pub datetime: String,
    pub uuid: String,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct ForumPost {
    pub chain_uuid: String,
    pub posts: Vec<Post>,
}

impl ForumPost {
    pub fn new(uuid: String, init: Post) -> ForumPost {
        ForumPost { chain_uuid: uuid, posts: vec![init] }
    }
}