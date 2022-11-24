use std::fmt::Display;

use serde::{Deserialize, Serialize};

// ANCHOR: range
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Range {
    pub start_pos: usize,
    pub end_pos: usize,
}
// ANCHOR_END: range

// ANCHOR: display
impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start at: {},end at: {}", self.start_pos, self.end_pos)
    }
}
// ANCHOR_END: display

// ANCHOR: impl
impl Range {
    pub fn new(start_pos: usize, end_pos: usize) -> Self {
        Range { start_pos, end_pos }
    }
    // 获取当前定位
    pub fn index(&self) -> usize {
        if self.end_pos > self.start_pos {
            return self.end_pos;
        }
        self.start_pos
    }

    // 移动开始光标
    pub fn advance_start(&mut self) -> usize {
        self.start_pos += 1;
        self.start_pos
    }
}

// ANCHOR_END: impl
