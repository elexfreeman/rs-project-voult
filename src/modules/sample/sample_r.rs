#![allow(non_snake_case)]

pub mod SampleRouteR {
    pub mod SampleRoute {
        use serde::Deserialize;
        use serde::Serialize;

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub title: String,
            pub description: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Response {
            pub title: String,
            pub description: String,
        }
    }

    pub mod SampleAddUser {
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

    pub mod SampleGetUser {
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
