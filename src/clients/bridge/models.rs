// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;

use crate::types::models::ModelsList;

use super::Client;

#[cfg(feature = "client-bridge")]
impl Client {
    pub async fn list_models(&self) -> anyhow::Result<ModelsList> {
        self.post("models/ListModels", json!({})).await
    }
}
