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

impl Editor {}
