use bson::{to_bson, Document, Bson};
use serde::ser::Serialize;

pub fn encode<T: Serialize>(doc: &T) -> Option<Document> {
    match to_bson(doc) {
        Ok(Bson::Document(d)) => Some(d),
        _ => None,
    }
}