use serde::{Deserialize,Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Text,
    File,
    FileChunk,
    FileComplete,
    Heartbeat,
    Ack,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    pub from: String,
    pub to: String,
    pub content: String,
    pub timestamp: i64,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl Message {
    pub fn new(
        msg_type: MessageType,
        from: String,
        to: String,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            msg_type,
            from,
            to,
            content,
            timestamp: chrono::Utc::now().timestamp(),
            metadata: HashMap::new(),
        }
    }

    pub fn text(from: String, to: String, content: String) -> Self {
        Self::new(MessageType::Text, from, to, content)
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key,value);
        self
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json(json: &str) -> anyhow::Result<Self>{
        Ok(serde_json::from_str(json)?)
    }
}