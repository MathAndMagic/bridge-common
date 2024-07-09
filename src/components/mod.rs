// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

pub mod chat_message;

use std::collections::HashMap;

use crate::types::{messages::Message, Result};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing required attribute: {0}")]
    MissingRequiredAttribute(String),
    #[error("attribute not found: {0}")]
    AttributeNotFound(String),
    #[error("incorrect value type, expected: {0:?}")]
    IncorrectValueType(AttributeType),
}

#[derive(Debug, Clone)]
pub enum AttributeType {
    Message,
    String,
}

#[derive(Debug, Clone)]
pub enum Value {
    Message(Message),
    String(String),
}

pub struct Attribute {
    pub _type: AttributeType,
    pub name: String,
    pub value: Option<Value>,
    pub is_required: bool,
}

pub enum Component {
    ChatMessage(chat_message::ChatMessage),
}

#[derive(Debug, Clone)]
pub struct Edge<'a> {
    pub from: &'a str,
    pub from_attribute: &'a str,
    pub to: &'a str,
    pub to_attribute: &'a str,
    pub value: Option<Value>,
}

pub struct Node<'a> {
    pub id: &'a str,
    pub component: Component,
    pub executed: bool,
}

pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub edges: Vec<Edge<'a>>,
    pub edges_from: HashMap<&'a str, Vec<Edge<'a>>>,
    pub edges_to: HashMap<&'a str, Vec<Edge<'a>>>,
}

impl<'a> Graph<'a> {
    pub fn new(nodes: Vec<Node<'a>>, edges: Vec<Edge<'a>>) -> Graph<'a> {
        let mut edges_from: HashMap<&'a str, Vec<Edge<'a>>> = HashMap::new();
        let mut edges_to: HashMap<&'a str, Vec<Edge<'a>>> = HashMap::new();

        for edge in &edges {
            edges_from
                .entry(edge.from)
                .or_insert_with(Vec::new)
                .push(edge.clone());
            edges_to
                .entry(edge.to)
                .or_insert_with(Vec::new)
                .push(edge.clone());
        }

        Graph {
            nodes,
            edges,
            edges_from,
            edges_to,
        }
    }

    pub fn candidate_ids(&self) -> Vec<&'a str> {
        self.nodes
            .iter()
            .filter(|node| {
                !node.executed && self.edges_to.get(node.id).unwrap_or(&Vec::new()).is_empty()
            })
            .map(|node| node.id)
            .collect()
    }

    pub fn fullfill_child_edges(&mut self, node_id: &str, attribute: &str, value: Value) {
        for edge in self.edges.iter_mut() {
            if edge.from == node_id && edge.from_attribute == attribute {
                edge.value = Some(value.clone());
            }
        }
    }

    pub fn execute(&mut self) -> Result<()> {
        loop {
            let candidate_ids = self.candidate_ids();
            if candidate_ids.is_empty() {
                break;
            }

            let mut actions = Vec::new();

            for id in &candidate_ids {
                if let Some(node) = self.nodes.iter_mut().find(|n| n.id == *id) {
                    let incoming_edges: Vec<&Edge> = self
                        .edges
                        .iter()
                        .filter(|edge| edge.to == node.id)
                        .collect();

                    for edge in &incoming_edges {
                        node.component
                            .set_attribute(edge.to_attribute, edge.value.clone().unwrap())?;
                    }

                    node.component.execute()?;

                    for output in node.component.outputs() {
                        actions.push((node.id, output.name.clone(), output.value.unwrap()));
                    }

                    node.executed = true;
                }
            }

            // Apply actions
            for (node_id, attr_name, value) in actions {
                self.fullfill_child_edges(node_id, &attr_name, value);
            }
        }

        Ok(())
    }
}

impl Component {
    pub fn execute(&mut self) -> Result<()> {
        match self {
            Component::ChatMessage(component) => component.execute(),
        }
    }

    pub fn set_attribute(&mut self, name: &str, value: Value) -> Result<()> {
        match self {
            Component::ChatMessage(component) => component.set_attribute(name, value),
        }
    }

    pub fn inputs(&mut self) -> Vec<Attribute> {
        match self {
            Component::ChatMessage(component) => component.inputs(),
        }
    }

    pub fn outputs(&mut self) -> Vec<Attribute> {
        match self {
            Component::ChatMessage(component) => component.outputs(),
        }
    }
}
