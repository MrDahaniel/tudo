use color_eyre::owo_colors::OwoColorize;
use ratatui::{prelude::*, widgets::*};
use ratatui::symbols

use serde::{Deserialize, Serialize};

use super::tasklist::TaskList;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Lists {
    tasklist: Vec<TaskList>,
}

impl Lists {}

impl Widget for List {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Lists ".on_truecolor(r, g, b));


    }
}
