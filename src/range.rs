use std::fmt::Display;

// ANCHOR: range
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    start_pos_index: usize,
    end_pos_index: usize,
    start_index: usize,
    start_line: usize,
    end_index: usize,
    end_line: usize,
}
// ANCHOR_END: range

// ANCHOR: display
impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {},col: {}", self.start_line, self.start_index)
    }
}
// ANCHOR_END: display

// ANCHOR: impl
impl Range {
    // 获取当前定位
    pub fn index(&self) -> usize {
        if self.end_pos_index > self.start_pos_index {
            return self.end_pos_index;
        }
        self.start_pos_index
    }

    // 移动开始光标
    pub fn advance_start(&mut self) -> usize {
        self.start_pos_index += 1;
        self.start_index += 1;
        self.start_pos_index
    }

    // 移动行
    pub fn advance_start_line(&mut self) {
        self.start_line += 1;
        self.start_index = 0;
    }

    // 移动结束行
    pub fn advance_end(&mut self) -> usize {
        self.end_pos_index += 1;
        self.end_index += 1;
        self.end_pos_index
    }
    // 移动结束光标
    pub fn advance_end_line(&mut self) {
        self.end_line += 1;
        self.end_index = 0;
    }
}

// ANCHOR_END: impl
