use crate::resources::auth::Auth;
use reqwest;

#[derive(Default)]
pub struct Client {
    pub(crate) api_key: String,
    pub(crate) secret_key: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
            client: reqwest::Client::new(),
        }
    }

    pub fn auth(&self) -> Auth {
        Auth::new(self)
    }
}
