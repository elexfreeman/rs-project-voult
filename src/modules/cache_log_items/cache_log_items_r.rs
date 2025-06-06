#![allow(non_snake_case)]

pub mod CacheLogItemsRouteR {
    pub mod Add {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub caption: String,
            pub price: f32,
            pub count: i32,
            pub cache_log_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub cache_log_items_id: i32,
        }
    }

    pub mod AddMany {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CacheLogItem {
            pub caption: String,
            pub price: f32,
            pub count: i32,
            pub cache_log_id: i32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub items: Vec<CacheLogItem>,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub cache_log_items_id: i32,
        }
    }

    pub mod Update {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub id: i32,
            pub caption: String,
            pub price: f32,
            pub count: i32,
            pub cache_log_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub cache_log_items_id: i32,
        }
    }

    pub mod Upsert {
        use crate::helpers::{empty_float, empty_int};
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CacheLogItem {
            #[serde(default = "empty_int")]
            pub id: i32,
            pub caption: String,
            #[serde(default = "empty_float")]
            pub price: f32,
            #[serde(default = "empty_int")]
            pub count: i32,
            pub cache_log_id: i32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub items: Vec<CacheLogItem>,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
        }
    }

    pub mod Get {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub id: i32,
            pub cache_log_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub id: i32,
            pub caption: String,
            pub price: f32,
            pub count: i32,
            pub cache_log_id: i32,
            pub created_at: String,
            pub updated_at: String,
        }
    }

    pub mod List {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CacheLogItems {
            pub id: i32,
            pub caption: String,
            pub price: f32,
            pub count: i32,
            pub cache_log_id: i32,
            pub created_at: String,
            pub updated_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub cache_log_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub list: Vec<CacheLogItems>,
        }
    }
}
