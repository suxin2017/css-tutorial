use crate::range::Range;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// ANCHOR: ast_tree
#[derive(Debug, Serialize, Deserialize)]
pub struct AstTree<T: Default + Serialize> {
    pub root: Option<Box<AstNode<T>>>,
}
// ANCHOR_END: ast_tree

// ANCHOR: ast_node_type
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AstNodeType<T: Default + Serialize>(pub T);
// ANCHOR_END: ast_node_type

// ANCHOR: ast_node
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AstNode<T: Default + Serialize> {
    pub node_type: AstNodeType<T>,
    pub range: Range,
    pub children: Option<Vec<AstNode<T>>>,
}
// ANCHOR_END: ast_node

impl<T: Default + Serialize> Default for AstTree<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

// ANCHOR:  ast_tree_builder
#[derive(Debug)]
pub struct AstTreeBuilder<T: Debug + Default + Serialize> {
    pub ast_tree: AstTree<T>,
    // uszie 标记子数组开始
    pub parent: Vec<(AstNode<T>, usize)>,
    pub children: Vec<AstNode<T>>,
}
// ANCHOR_END:  ast_tree_builder

impl<T: Default + Serialize> AstTree<T> {
    pub fn travel<F: Fn(&AstNode<T>)>(&self, cb: &F) {
        match &self.root {
            Some(node) => node.travel(&cb),
            None => {}
        }
    }
}

impl<T: Default + Serialize> AstNode<T> {
    pub fn travel<F: Fn(&AstNode<T>)>(&self, cb: &F) {
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
    // ANCHOR:  impl
    // 开始一个token节点
    pub fn start_node<N: Into<AstNode<T>>>(&mut self, node: N) {
        self.parent.push((node.into(), self.children.len()))
    }
    // 插入一个token
    pub fn token<N: Into<AstNode<T>>>(&mut self, node: N) {
        self.children.push(node.into());
    }
    // 关闭一个token节点
    pub fn finish_node(&mut self) {
        let (mut parent, last_pos) = self.parent.pop().unwrap();

        let children = self.children.split_off(last_pos);
        let start_child = children.first();
        let end_child = children.last();

        parent.range = Range::new(
            start_child.unwrap_or(&AstNode::default()).range.start_pos,
            end_child.unwrap_or(&AstNode::default()).range.end_pos,
        );

        parent.children = Some(children);
        self.children.push(parent);
    }
    // 将根节点挂载到ast tree中
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
    // ANCHOR_END:  impl
}
