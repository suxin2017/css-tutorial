use serde::{Deserialize, Serialize};

// ANCHOR: range
#[derive(Default, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Range {
    pub start_pos: usize,
    pub end_pos: usize,
}
// ANCHOR_END: range

// ANCHOR: impl
impl Range {
    pub fn new(start_pos: usize, end_pos: usize) -> Self {
        Range { start_pos, end_pos }
    }
}

// ANCHOR_END: impl
