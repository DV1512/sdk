use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use api_forge::Request;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod wasm;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Id {
    pub id: String,
    pub tb: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[request(endpoint = "/api/v1/oauth/token", transmission = FormData, response_type = TokenResponse, method = POST)]
pub struct Login {
    pub grant_type: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

// make the fucking get user by request or else
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[request(endpoint = "/api/v1/user", response_type = User, authentication = Bearer)]
pub struct GetUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    pub id: Id,
    pub email: String,
    pub url_safe_username: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub picture: Option<String>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone, Default)]
pub enum Role {
    Owner,
    Admin,
    #[default]
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[request(endpoint = "/posts", response_type = Posts)]
pub struct GetPosts;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Request)]
#[request(endpoint = "/posts", response_type = Post, method = POST, transmission = Json)]
pub struct CreatePost {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub title: String,
    pub body: String,

    #[request(header_name = "test")]
    #[serde(skip)]
    header: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Request)]
#[request(endpoint = "/posts/{id}", response_type = EmptyResponse, method = DELETE, path_parameters = ["id"])]
pub struct DeletePost {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct Posts(Vec<Post>);

impl Deref for Posts {
    type Target = Vec<Post>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Posts {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub struct EmptyResponse(HashMap<String, String>);

#[cfg(test)]
mod tests {
    use super::*;
    use api_forge::ApiRequest;

    #[tokio::test]
    async fn get_user() {
        // Initialize the request.
        let request = GetUser {
            username: None,
            email: Some("emil.schutt@gmail.com".to_string()),
            token: None,
        };

        // Send the request and await the response.
        let result = request
            .send_and_parse("http://localhost:9999", None, Some(("test".to_string(), None)))
            .await;

        match result {
            Ok(response) => {
                println!("Successfully fetched user: {:?}", response)
            }
            Err(e) => eprintln!("Error occurred: {:?}", e),
        }
    }

    #[tokio::test]
    async fn login() {
        // Initialize the request.
        let request = Login {
            grant_type: "password".to_string(),
            username: "emil.schutt@gmail.com".to_string(),
            password: "test".to_string(),
        };

        // Send the request and await the response.
        let result = request
            .send_and_parse("http://localhost:9999", None, None)
            .await;

        match result {
            Ok(token_response) => {
                println!("Successfully logged in: {:?}", token_response)
            }
            Err(e) => eprintln!("Error occurred: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test() {
        // Initialize the request.
        let request = GetPosts;

        // Define the base URL (e.g., JSONPlaceholder API for testing).
        let base_url = "https://jsonplaceholder.typicode.com";

        // Send the request and await the response.
        let result = request.send_and_parse(base_url, None, None).await;

        match result {
            Ok(post) => println!("Successfully fetched post: {:?}", post),
            Err(e) => eprintln!("Error occurred: {:?}", e),
        }

        // Initialize the request.
        let request = CreatePost {
            user_id: 1,
            title: "Test".to_string(),
            body: "Test".to_string(),
            header: Some("test-header".to_string()),
        };

        // Send the request and await the response.
        let result = request.send_and_parse(base_url, None, None).await;

        match result {
            Ok(post) => println!("Successfully created post: {:?}", post),
            Err(e) => eprintln!("Error occurred: {:?}", e),
        }

        // Initialize the request.
        let request = DeletePost { id: 100 };

        // Send the request and await the response.
        let result = request.send_and_parse(base_url, None, None).await;

        match result {
            Ok(post) => println!("Successfully deleted post: {:?}", post),
            Err(e) => eprintln!("Error occurred: {:?}", e),
        }
    }
}
