use crate::range::Range;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct AstTree<T: Default + Serialize> {
    pub root: Option<Box<AstNode<T>>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AstNodeType<T: Default + Serialize>(pub T);

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AstNode<T: Default + Serialize> {
    pub node_type: AstNodeType<T>,
    pub range: Range,
    pub children: Option<Vec<AstNode<T>>>,
}

impl<T: Default + Serialize> Default for AstTree<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

#[derive(Debug)]
pub struct AstTreeBuilder<T: Debug + Default + Serialize> {
    pub ast_tree: AstTree<T>,
    // uszie 标记子数组开始
    parent: Vec<(AstNode<T>, usize)>,
    children: Vec<AstNode<T>>,
}

impl<T: Default + Serialize> AstTree<T> {
    fn travel<F: Fn(&AstNode<T>)>(&self, cb: &F) {
        match &self.root {
            Some(node) => node.travel(&cb),
            None => {}
        }
    }
}

impl<T: Default + Serialize> AstNode<T> {
    fn travel<F: Fn(&AstNode<T>)>(&self, cb: &F) {
        cb(&self);
        if let Some(children) = &self.children {
            children.iter().for_each(|child| child.travel(cb))
        }
    }
}

impl<T: Debug + Default + Serialize> AstTreeBuilder<T> {
    pub fn new() -> Self {
        AstTreeBuilder {
            ast_tree: Default::default(),
            parent: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn start_node<N: Into<AstNode<T>>>(&mut self, node: N) {
        self.parent.push((node.into(), self.children.len()))
    }
    pub fn token<N: Into<AstNode<T>>>(&mut self, node: N) {
        self.children.push(node.into());
    }
    pub fn finish_node(&mut self) {
        let (mut parent, last_pos) = self.parent.pop().unwrap();

        let children = self.children.split_off(last_pos);
        let start_child = children.first();
        let end_child = children.last();

        parent.range = Range::new(
            start_child
                .unwrap_or(&AstNode::default())
                .range
                .start_pos_index,
            end_child.unwrap_or(&AstNode::default()).range.end_pos_index,
        );

        parent.children = Some(children);
        self.children.push(parent);
    }
    pub fn finish(&mut self) {
        if self.children.len() > 1 {
            panic!("存在非闭合子节点")
        }
        if self.parent.len() > 0 {
            dbg!(&self.parent);
            panic!("存在非闭合夫节点")
        }
        self.ast_tree.root = Some(Box::new(self.children.pop().unwrap()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    enum SynataxNodeType {
        A = 1,
        B,
        C,
    }

    impl Default for SynataxNodeType {
        fn default() -> Self {
            SynataxNodeType::A
        }
    }

    impl From<SynataxNodeType> for AstNodeType<SynataxNodeType> {
        fn from(token: SynataxNodeType) -> Self {
            Self(token)
        }
    }

    impl From<SynataxNodeType> for AstNode<SynataxNodeType> {
        fn from(token: SynataxNodeType) -> Self {
            Self {
                range: Range {
                    start_pos_index: token as usize,
                    end_pos_index: token as usize * 2,
                },
                node_type: token.into(),
                children: None,
            }
        }
    }

    #[test]
    fn test_name() {
        let mut ast_tree_builder = AstTreeBuilder::new();
        ast_tree_builder.start_node(SynataxNodeType::A); // A
        ast_tree_builder.token(SynataxNodeType::B); //   B
        ast_tree_builder.start_node(SynataxNodeType::A); //   A
        ast_tree_builder.token(SynataxNodeType::B); //     B
        ast_tree_builder.finish_node();
        ast_tree_builder.finish_node();
        ast_tree_builder.finish();
        let serialized = serde_json::to_string(&ast_tree_builder.ast_tree).unwrap();
        println!("serialized = {}", serialized);
        dbg!(ast_tree_builder.ast_tree);
    }

    #[test]
    fn test_travel() {
        let mut ast_tree_builder = AstTreeBuilder::new();
        ast_tree_builder.start_node(SynataxNodeType::A); // A
        ast_tree_builder.token(SynataxNodeType::B); //   B
        ast_tree_builder.start_node(SynataxNodeType::A); //   A
        ast_tree_builder.token(SynataxNodeType::B); //     B
        ast_tree_builder.finish_node();
        ast_tree_builder.finish_node();
        ast_tree_builder.finish();
        ast_tree_builder
            .ast_tree
            .travel(&|node| println!("{:?}\n", node))
    }
}
