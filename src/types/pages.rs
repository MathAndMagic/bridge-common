// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Page {
    pub id: Uuid,
    pub company_id: Uuid,
    pub title: String,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ShortPage {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
pub struct PagesList {
    pub pages: Vec<ShortPage>,
}
