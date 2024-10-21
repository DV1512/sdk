use crate::wasm::Id;
use api_forge::ApiRequest;
use chrono::{DateTime, Datelike, Timelike, Utc};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct User {
    id: Id,
    email: String,
    url_safe_username: String,
    username: String,
    first_name: String,
    last_name: String,
    created_at: js_sys::Date,
    last_login: Option<js_sys::Date>,
    picture: Option<String>,
    role: Role,
}

fn from_chrono_to_js(date: DateTime<Utc>) -> js_sys::Date {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let hours = date.hour();
    let minutes = date.minute();
    let seconds = date.second();
    js_sys::Date::new_with_year_month_day_hr_min_sec(
        year as u32,
        month as i32,
        day as i32,
        hours as i32,
        minutes as i32,
        seconds as i32,
    )
}

#[wasm_bindgen]
impl User {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: Id,
        email: String,
        url_safe_username: String,
        username: String,
        first_name: String,
        last_name: String,
        created_at: js_sys::Date,
        last_login: Option<js_sys::Date>,
        picture: Option<String>,
        role: Role,
    ) -> User {
        User {
            id,
            email,
            url_safe_username,
            username,
            first_name,
            last_name,
            created_at,
            last_login,
            picture,
            role,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> Id {
        self.id.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn email(&self) -> String {
        self.email.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn url_safe_username(&self) -> String {
        self.url_safe_username.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn username(&self) -> String {
        self.username.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn created_at(&self) -> js_sys::Date {
        self.created_at.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn last_login(&self) -> Option<js_sys::Date> {
        self.last_login.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn picture(&self) -> Option<String> {
        self.picture.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn role(&self) -> Role {
        self.role
    }
}

impl From<&mut super::super::User> for User {
    fn from(user: &mut super::super::User) -> Self {
        let created_at = from_chrono_to_js(user.created_at);
        let last_login = user.last_login.map(from_chrono_to_js);

        User {
            id: user.id.clone().into(),
            email: user.email.clone(),
            url_safe_username: user.url_safe_username.clone(),
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            created_at,
            last_login,
            picture: user.picture.clone(),
            role: user.role.clone().into(),
        }
    }
}

impl From<super::super::User> for User {
    fn from(user: super::super::User) -> Self {
        let created_at = from_chrono_to_js(user.created_at);
        let last_login = user.last_login.map(from_chrono_to_js);

        Self {
            id: user.id.into(),
            email: user.email,
            url_safe_username: user.url_safe_username,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            created_at,
            last_login,
            picture: user.picture,
            role: user.role.into(),
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum Role {
    Owner,
    Admin,
    User,
}

impl From<super::super::Role> for Role {
    fn from(role: super::super::Role) -> Self {
        match role {
            super::super::Role::Owner => Role::Owner,
            super::super::Role::Admin => Role::Admin,
            super::super::Role::User => Role::User,
        }
    }
}

#[wasm_bindgen]
pub struct GetUserByFilter {
    username: Option<String>,
    email: Option<String>,
    token: Option<String>,
}

#[wasm_bindgen]
impl GetUserByFilter {
    #[wasm_bindgen(constructor)]
    pub fn new(
        username: Option<String>,
        email: Option<String>,
        token: Option<String>,
    ) -> GetUserByFilter {
        GetUserByFilter {
            username,
            email,
            token,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn token(&self) -> Option<String> {
        self.token.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_username(&mut self, username: Option<String>) {
        self.username = username;
    }

    #[wasm_bindgen(setter)]
    pub fn set_email(&mut self, email: Option<String>) {
        self.email = email;
    }

    #[wasm_bindgen(setter)]
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
}

#[wasm_bindgen(js_name = getUser)]
pub async fn get_user(filter: GetUserByFilter) -> Result<User, String> {
    let request = crate::GetUser {
        username: filter.username,
        email: filter.email,
        token: filter.token,
    };

    let user = request
        .send_and_parse("http://localhost:9999", None, None)
        .await
        .map_err(|err| format!("{:?}", err))?;

    Ok(user.into())
}
