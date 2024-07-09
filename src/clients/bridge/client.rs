// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
#[cfg(feature = "client-bridge")]
use gloo_net::http::Request;

#[derive(Debug, Clone)]
pub struct Client {
    pub api_key: String,
    pub api_url: String,
    pub user_agent: String,
}

#[cfg(feature = "client-bridge")]
impl<'a> Client {
    #[must_use]
    pub fn new(api_key: &'a str, api_url: &'a str, user_agent: &'a str) -> Self {
        Self {
            api_key: api_key.to_string(),
            api_url: api_url.to_string(),
            user_agent: user_agent.to_string(),
        }
    }

    pub async fn post<T, B>(&self, endpoint: &str, body: B) -> anyhow::Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize,
    {
        let url = format!("{}{endpoint}", self.api_url);

        Ok(Request::post(&url)
            .header("Authorization", &format!("Bearer {}", self.api_key))
            .header("User-Agent", &self.user_agent)
            .json(&body)
            .with_context(|| "Failed to serialize request")?
            .send()
            .await
            .with_context(|| "Failed to send request")?
            .json()
            .await
            .with_context(|| "Failed to deserialize response")?)
    }
}
