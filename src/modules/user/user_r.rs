#![allow(non_snake_case)]


pub mod UserRouteR {
    pub mod Add {
        use serde::Deserialize;
        use serde::Serialize;
        use crate::helpers::empty_string;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub id: i32,
            #[serde(default = "empty_string")]
            pub first_name: String,
            #[serde(default = "empty_string")]
            pub last_name: String,
            pub username: String,
            #[serde(default = "empty_string")]
            pub language_code: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub user_id: i32,
        }
    }

    pub mod UserAddUser {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub first_name: String,
            pub last_name: String,
            pub username: String,
            pub email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub id: String,
        }
    }

    pub mod UserGetUser {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub username: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub first_name: String,
            pub last_name: String,
            pub username: String,
            pub email: String,
        }
    }
}
