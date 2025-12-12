use crate::resources::auth::Auth;
use crate::resources::transactions::Transaction;
use reqwest;
use std::sync::RwLock;

#[derive(Default)]
pub struct MonnfiyClient {
    pub(crate) api_key: String,
    pub(crate) secret_key: String,
    pub(crate) access_token: RwLock<Option<String>>,
    pub(crate) client: reqwest::Client,
}

impl MonnfiyClient {
    pub fn new(api_key: String, secret_key: String) -> Self {
        Self {
            api_key,
            secret_key,
            access_token: RwLock::new(None),
            client: reqwest::Client::new(),
        }
    }

    pub fn auth(&self) -> Auth<'_> {
        Auth::new(self)
    }

    pub fn transaction(&self) -> Transaction<'_> {
        Transaction::new(self)
    }
}
