use std::fmt::Display;


#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    start_pos_index: usize,
    end_pos_index: usize,
    start_index: usize,
    start_line: usize,
    end_index: usize,
    end_line: usize,
}
impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {},col: {}", self.start_line, self.start_index)
    }
}
impl Range {
    pub fn index(&self) -> usize {
        if self.end_pos_index > self.start_pos_index {
            return self.end_pos_index;
        }
        self.start_pos_index
    }

    pub fn advance_start(&mut self) -> usize {
        self.start_pos_index += 1;
        self.start_index += 1;
        self.start_pos_index
    }

    pub fn advance_start_line(&mut self) {
        self.start_line += 1;
        self.start_index = 0;
    }

    pub fn advance_end(&mut self) -> usize {
        self.end_pos_index += 1;
        self.end_index += 1;
        self.end_pos_index
    }
    pub fn advance_end_line(&mut self) {
        self.end_line += 1;
        self.end_index = 0;
    }
}