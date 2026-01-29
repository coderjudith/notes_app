use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
}

impl Note {
    pub fn new(title: String, content: String, tags: Vec<String>) -> Self {
        let now = Local::now().to_rfc3339();
        Note {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            created_at: now.clone(),
            updated_at: now,
            tags,
        }
    }

    pub fn update(
        &mut self,
        title: Option<String>,
        content: Option<String>,
        tags: Option<Vec<String>>,
    ) {
        if let Some(t) = title {
            self.title = t;
        }
        if let Some(c) = content {
            self.content = c;
        }
        if let Some(tags) = tags {
            self.tags = tags;
        }
        self.updated_at = Local::now().to_rfc3339();
    }
}
