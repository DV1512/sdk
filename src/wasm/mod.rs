#![cfg(feature = "wasm")]

mod login;

use api_forge::ApiRequest;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Post {
  user_id: i32,
  id: i32,
  title: String,
  body: String,
}

#[wasm_bindgen]
impl Post {
    #[wasm_bindgen(constructor)]
    pub fn new(user_id: i32, id: i32, title: String, body: String) -> Post {
        Post {
            user_id,
            id,
            title,
            body,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.title.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn body(&self) -> String {
        self.body.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    #[wasm_bindgen(setter)]
    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

impl From<super::Post> for Post {
    fn from(post: super::Post) -> Self {
        Post {
            user_id: post.user_id,
            id: post.id,
            title: post.title,
            body: post.body,
        }
    }
}

impl From<&mut super::Post> for Post {
    fn from(post: &mut super::Post) -> Self {
        Post {
            user_id: post.user_id,
            id: post.id,
            title: post.title.clone(),
            body: post.body.clone(),
        }
    }
}

#[wasm_bindgen(js_name = getPosts)]
pub async fn get_posts() -> Result<Vec<Post>, String> {
  let mut posts = super::GetPosts.send_and_parse("https://jsonplaceholder.typicode.com", None, None).await.map_err(|err| format!("{:?}", err))?;

  let posts: Vec<Post> = posts.iter_mut().map(|post| post.into()).collect();

  Ok(posts)
}