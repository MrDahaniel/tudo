use ratatui::prelude::*;
use ratatui::{widgets::Widget, Frame};

pub enum EditorScreen {
    Main,
    TaskLists,
    List(u32),
    Exiting,
}

pub enum EditorMode {}

pub struct Editor {
    mode: Option<EditorMode>,
}

impl Widget for &Editor {
    fn render(self, area: Rect, buf: &mut Buffer) {
        todo!()
    }
}

impl Editor {
    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }
}
