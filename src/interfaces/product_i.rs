#![allow(non_snake_case)]
use serde::Deserialize;
use serde::Serialize;

use crate::interfaces::common_i::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct WhyWeFields {
    title: String,
    description: String,
    image: Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopMenusFields {
    #[serde(default = "empty_string")]
    caption: String,
    #[serde(default = "empty_string")]
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPageFields {
    #[serde(default = "empty_string")]
    title: String,
    #[serde(default = "empty_string")]
    description: String,
    #[serde(default = "empty_string")]
    phone: String,
    #[serde(default = "empty_string")]
    address: String,
    #[serde(default = "empty_string")]
    instagrammLink: String,
    #[serde(default = "empty_string")]
    aboutTitle: String,
    #[serde(default = "empty_string")]
    aboutText: String,
    #[serde(default = "empty_string")]
    productListTitle: String,
    #[serde(default = "empty_string")]
    whyWeTitle: String,
    #[serde(default = "empty_string")]
    createdAt: String,
    #[serde(default = "empty_string")]
    updatedAt: String,
    #[serde(default = "empty_string")]
    productAddDataTitle: String,
    #[serde(default = "empty_string")]
    HeaderText: String,
    #[serde(default = "empty_string")]
    galeryTitle: String,
    logo: Image,
    aboutImg: ImageList,
    galeryImage: ImageList,
    logoTitle: Image,
    phoneIco: Image,
    locationIco: Image,
    instaIco: Image,
    whyWe: Vec<WhyWeFields>,
    top_menus: DataStru<TopMenusFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPageI {
    data: DataAttributesStru<ProductPageFields>,
}
