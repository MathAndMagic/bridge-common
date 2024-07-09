// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;

use crate::types::agents::AgentsList;

use super::Client;

#[cfg(feature = "client-bridge")]
impl Client {
    pub async fn list_agents(&self) -> anyhow::Result<AgentsList> {
        self.post("agents/ListAgents", json!({})).await
    }
}
