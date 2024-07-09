// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

use super::{Attribute, AttributeType, Value};

use crate::{
    components::Error,
    types::{messages::Message, Result},
};

#[derive(Debug, Clone, Default)]
pub struct ChatMessage {
    message: Option<Message>,
    text: Option<String>,
}

impl ChatMessage {
    pub fn execute(&mut self) -> Result<()> {
        self.text = match &self.message {
            Some(message) => message.content.clone(),
            None => return Err(Error::MissingRequiredAttribute("message".to_string()).into()),
        };

        Ok(())
    }

    pub fn set_attribute(&mut self, name: &str, value: Value) -> Result<()> {
        match name {
            "message" => match value {
                Value::Message(message) => self.message = Some(message),
                _ => return Err(Error::IncorrectValueType(AttributeType::Message).into()),
            },
            _ => return Err(Error::AttributeNotFound(name.to_string()).into()),
        }

        Ok(())
    }

    pub fn inputs(&self) -> Vec<Attribute> {
        vec![Attribute {
            _type: AttributeType::Message,
            name: "message".to_string(),
            value: self.message.clone().map(Value::Message),
            is_required: true,
        }]
    }

    pub fn outputs(&self) -> Vec<Attribute> {
        vec![Attribute {
            _type: AttributeType::String,
            name: "text".to_string(),
            value: self.text.clone().map(Value::String),
            is_required: false,
        }]
    }
}
