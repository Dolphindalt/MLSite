use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;

#[derive(Clone, Debug, RustcEncodable, RustcDecodable, Serialize, Deserialize)]
pub struct NewsPost {
    pub title: String,
    pub body: String,
    pub author: String,
    pub datetime: String,
    pub uuid: String,
}

impl NewsPost {
    pub fn new(title: &str, body: &str, author: &str, datetime: DateTime<UTC>, uuid: Uuid) -> NewsPost {
        NewsPost {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            datetime: datetime.to_string(),
            uuid: uuid.to_string(),
        }
    }

    pub fn uuid(&self) -> Uuid {
        Uuid::parse_str(&self.uuid).unwrap()
    }
}