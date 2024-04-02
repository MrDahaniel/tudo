use ratatui::symbols::border;
use ratatui::{prelude::*, widgets::*};

use serde::{Deserialize, Serialize};

use super::tasklist::TaskList;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct Lists {
    pub tasklist: Vec<TaskList>,
}

impl Lists {}

impl Widget for Lists {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = block::Title::from(" Lists ".on_white().black().bold());
        let block = Block::default()
            .title(title.alignment(Alignment::Left))
            .borders(Borders::ALL)
            .border_set(border::PLAIN);

        let list = List::default()
            .items(self.tasklist.iter().map(|k| k.name.clone()).into_iter())
            .block(block);
    }
}
