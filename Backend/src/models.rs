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
    pub username: String,
    pub hashword: String,
    pub date_created: String,
}

#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub username: String,
    pub hashword: String,
}