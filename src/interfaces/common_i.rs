#![allow(non_snake_case)]
use serde::Deserialize;
use serde::Serialize;

pub fn empty_string() -> std::string::String {
    String::from("")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataAttributesStru<T> {
    attributes: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStru<T> {
    data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageFields {
    #[serde(default = "empty_string")]
    name: String,
    #[serde(default = "empty_string")]
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    data: DataAttributesStru<ImageFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageList {
    data: Vec<DataAttributesStru<ImageFields>>,
}
