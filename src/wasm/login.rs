use api_forge::ApiRequest;
use wasm_bindgen::prelude::*;
use crate::Login;

#[wasm_bindgen]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: usize,
}

#[wasm_bindgen]
impl TokenResponse {
    #[wasm_bindgen(constructor)]
    pub fn new(access_token: String, refresh_token: String, token_type: String, expires_in: usize) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type,
            expires_in,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn access_token(&self) -> String {
        self.access_token.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn refresh_token(&self) -> String {
        self.refresh_token.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn token_type(&self) -> String {
        self.token_type.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn expires_in(&self) -> usize {
        self.expires_in
    }
}

impl From<super::super::TokenResponse> for TokenResponse {
    fn from(token: super::super::TokenResponse) -> Self {
        Self {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            token_type: token.token_type,
            expires_in: token.expires_in,
        }
    }
}

impl From<&mut super::super::TokenResponse> for TokenResponse {
    fn from(token: &mut super::super::TokenResponse) -> Self {
        Self {
            access_token: token.access_token.clone(),
            refresh_token: token.refresh_token.clone(),
            token_type: token.token_type.clone(),
            expires_in: token.expires_in,
        }
    }
}

#[wasm_bindgen(js_name = passwordLogin)]
pub async fn password_login(username: String, password: String) -> Result<TokenResponse, String> {
    let request = Login {
        grant_type: "password".to_string(),
        username,
        password,
    };

    let token = request.send_and_parse("http://localhost:9999", None, None).await.map_err(|err| format!("{:?}", err))?;

    Ok(token.into())
}
