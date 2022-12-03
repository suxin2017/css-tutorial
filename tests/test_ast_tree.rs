#[cfg(test)]
mod test_ast_tree {
    use css_tutorial::{
        ast::{AstNode, AstNodeType, AstTreeBuilder},
        range::Range,
    };
    use serde::{Deserialize, Serialize};

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
                    start_pos: token as usize,
                    end_pos: token as usize * 2,
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

    // TODO: 遍历ast tree 节点
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
