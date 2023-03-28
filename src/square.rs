#[derive(Clone, Default)]
pub struct Square {
    pub value: String,
    pub show_text: bool,
    pub solved_cell: bool,
    pub focus: bool,
}