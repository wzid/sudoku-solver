#[derive(Clone)]
pub struct Square {
    pub value: String,
    pub show_text: bool,
    pub solved_cell: bool,
    pub focus: bool,
}

impl Square {
    pub fn new() -> Self {
        Self {
            value: String::new(),
            show_text: false,
            solved_cell: false,
            focus: false
        }
    }
}