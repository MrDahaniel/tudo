use serde::{Deserialize, Serialize};

use ratatui::prelude::*;
use ratatui::text::Line;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) enum TaskStatus {
    #[default]
    Pending,
    Completed,
    Delayed,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) enum Urgency {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) enum Complexity {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) enum Priority {
    #[default]
    Low,
    Medium,
    High,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct Task {
    pub description: String,
    pub status: TaskStatus,
    pub urgency: Urgency,
    pub complexity: Complexity,
    pub priority: Priority,
    pub tasks: Vec<Task>,
}

impl Task {
    pub fn new(description: &str) -> Self {
        Self {
            description: description.to_owned(),
            ..Default::default()
        }
    }
}
