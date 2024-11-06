#![allow(non_snake_case)]

pub mod CacheLogRouteR {
    pub mod Add {
        use crate::helpers::empty_string;
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub contractor_id: i32,
            pub project_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub cache_log_id: i32,
        }
    }

    pub mod Update {
        use crate::helpers::empty_string;
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub id: i32,
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub project_id: i32,
            pub contractor_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub cache_log_id: i32,
        }
    }

    pub mod Get {
        use serde::Deserialize;
        use serde::Serialize;
        use crate::helpers::empty_string;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub id: i32,
            pub project_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub id: i32,
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub contractor_id: i32,
            pub created_at: String,
            pub project_id: i32,
        }
    }

    pub mod List {
        use serde::Deserialize;
        use serde::Serialize;
        use crate::helpers::empty_string;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct CacheLog {
            pub id: i32,
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub created_at: String,
            pub contractor_id: i32,
            pub project_id: i32,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub project_id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub list: Vec<CacheLog>,
        }
    }
}
