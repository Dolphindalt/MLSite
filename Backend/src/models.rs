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