// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Application(#[from] anyhow::Error),

    #[cfg(feature = "database")]
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[cfg(feature = "abilities")]
    #[error(transparent)]
    Abilities(#[from] crate::abilities::Error),
    #[cfg(feature = "database")]
    #[error(transparent)]
    Database(#[from] crate::database::Error),
    #[error("feedback channel error: {0}")]
    Channel(anyhow::Error),
    #[error("component error: {0}")]
    Components(#[from] crate::components::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[cfg(feature = "browser")]
    #[error(transparent)]
    Browser(#[from] crate::browser::Error),
    #[cfg(feature = "docker")]
    #[error(transparent)]
    Docker(#[from] crate::docker::Error),
    #[cfg(feature = "embeddings")]
    #[error("embeddings error: {0}")]
    Embeddings(#[from] crate::embeddings::Error),
    #[cfg(feature = "tasks")]
    #[error(transparent)]
    Executor(#[from] crate::task_executor::Error),
    #[cfg(feature = "messages")]
    #[error(transparent)]
    Messages(#[from] crate::messages::Error),
    #[cfg(feature = "models")]
    #[error(transparent)]
    Models(#[from] crate::models::Error),
    #[error(transparent)]
    Pages(#[from] crate::pages::Error),
    #[cfg(feature = "tasks")]
    #[error(transparent)]
    Planner(#[from] crate::task_planner::Error),
    #[error(transparent)]
    Settings(#[from] crate::settings::Error),
    #[cfg(feature = "browser")]
    #[error(transparent)]
    WebBrowsing(#[from] crate::tools::web_browsing::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(format!("{self:#}").as_str())
    }
}
