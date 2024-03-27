use serde::{Deserialize, Serialize};

use super::task::Task;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(super) struct TaskList {
    name: String,
    tasks: Vec<Task>,
}

impl TaskList {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}
