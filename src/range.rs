use std::fmt::Display;

// ANCHOR: range
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
   pub start_pos_index: usize,
    pub end_pos_index: usize,
}
// ANCHOR_END: range

// ANCHOR: display
impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start at: {},end at: {}", self.start_pos_index,self.end_pos_index)
    }
}
// ANCHOR_END: display

// ANCHOR: impl
impl Range {
    pub fn new(start_pos: usize,end_pos:usize)->Self{
        Range { start_pos_index: start_pos, end_pos_index: end_pos }
    }
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
        self.start_pos_index
    }
   
}

// ANCHOR_END: impl
