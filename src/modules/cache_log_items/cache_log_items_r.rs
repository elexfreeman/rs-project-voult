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
