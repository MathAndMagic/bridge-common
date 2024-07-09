// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::types::chats::Chat;

use super::Client;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ChatsList {
    pub chats: Vec<Chat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChat {
    pub agent_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListChats {
    pub is_pinned: Option<bool>,
}

#[cfg(feature = "client-bridge")]
impl Client {
    pub async fn create_chat(&self, req: &CreateChat) -> anyhow::Result<Chat> {
        self.post("chats/CreateChat", req).await
    }

    pub async fn list_chats(&self) -> anyhow::Result<ChatsList> {
        self.post("chats/ListChats", json!({})).await
    }
}
