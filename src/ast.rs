use std::fmt::{self, write, Display};

#[derive(Debug)]
pub struct AstTree {
    root: Option<Box<AstNode>>,
}
#[derive(Debug)]
pub struct AstNodeType(pub u16);

#[derive(Debug)]
pub struct AstNode {
    node_type: AstNodeType,
    children: Option<Vec<AstNode>>,
}

impl Default for AstTree {
    fn default() -> Self {
        Self { root: None }
    }
}

#[derive(Debug)]
struct AstTreeBuilder {
    ast_tree: AstTree,
    // uszie 标记子数组开始
    parent: Vec<(AstNode, usize)>,
    children: Vec<AstNode>,
}

impl AstTreeBuilder {
    pub fn new() -> Self {
        AstTreeBuilder {
            ast_tree: Default::default(),
            parent: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn start_node(&mut self, node_type: AstNodeType) {
        self.parent.push((
            AstNode {
                node_type,
                children: None,
            },
            self.children.len(),
        ))
    }
    pub fn token(&mut self, node_type: AstNodeType) {
        self.children.push(AstNode {
            node_type,
            children: None,
        })
    }
    pub fn finish_node(&mut self) {
        let (mut parent, last_pos) = self.parent.pop().unwrap();
        let children = self.children.split_off(last_pos);
        parent.children = Some(children);
        self.children.push(parent);
    }
    pub fn finish(&mut self) {
        if self.children.len() > 1 {
            panic!("存在非闭合子节点")
        }
        if self.parent.len() > 0 {
            panic!("存在非闭合夫节点")
        }
        self.ast_tree.root = Some(Box::new(self.children.pop().unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use crate::ast;

    use super::*;

    enum SynataxNodeType {
        A = 1,
        B,
        C,
    }

    impl From<SynataxNodeType> for AstNodeType {
        fn from(token: SynataxNodeType) -> Self {
            Self(token as u16)
        }
    }

    #[test]
    fn test_name() {
        let mut ast_tree_builder = AstTreeBuilder::new();
        ast_tree_builder.start_node(SynataxNodeType::A.into()); // A
        ast_tree_builder.token(SynataxNodeType::B.into()); //   B
        ast_tree_builder.start_node(SynataxNodeType::A.into()); //   A
        ast_tree_builder.token(SynataxNodeType::B.into()); //     B
        ast_tree_builder.finish_node();
        ast_tree_builder.finish_node();
        ast_tree_builder.finish();
        dbg!(ast_tree_builder.ast_tree);
    }
}
