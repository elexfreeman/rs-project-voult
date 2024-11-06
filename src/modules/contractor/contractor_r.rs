#![allow(non_snake_case)]

pub mod ContractorRouteR {
    pub mod Add {
        use crate::helpers::empty_string;
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub contractor_id: i32,
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
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub contractor_id: i32,
        }
    }

    pub mod Get {
        use serde::Deserialize;
        use serde::Serialize;
        use crate::helpers::empty_string;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub id: i32,
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub id: i32,
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub created_at: String,
        }
    }

    pub mod List {
        use serde::Deserialize;
        use serde::Serialize;
        use crate::helpers::empty_string;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Contractor {
            pub id: i32,
            pub caption: String,
            #[serde(default = "empty_string")]
            pub description: String,
            pub created_at: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub auth: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub list: Vec<Contractor>,
        }
    }
}
