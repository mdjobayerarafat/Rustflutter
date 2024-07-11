use serde::{Serializer, Deserializer};

#[derive(Serializ, Deserializer, Debug)]

pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub date: String,
    pub photo_link: String,
}