# 语法树结构定义

我们将用一棵 ast tree 来表达语法结构。

比如 paser 下面的 css 规则，他的 ast 结构 json 表示是下面这样的。

```css
.glyphicon-chevron-up:before {
  content: "\e113";
}
```

```json
{
  "root": {
    "node_type": "Stylesheet",
    "range": { "start_pos": 13, "end_pos": 90 },
    "children": [
      {
        "node_type": "RuleList",
        "range": { "start_pos": 13, "end_pos": 90 },
        "children": [
          {
            "node_type": "Selector",
            "range": { "start_pos": 13, "end_pos": 41 },
            "children": [
              {
                "node_type": "SimpleSelect",
                "range": { "start_pos": 13, "end_pos": 41 },
                "children": [
                  {
                    "node_type": "Class",
                    "range": { "start_pos": 13, "end_pos": 34 },
                    "children": [
                      {
                        "node_type": "Dot",
                        "range": { "start_pos": 13, "end_pos": 14 },
                        "children": null
                      },
                      {
                        "node_type": "IdentToken",
                        "range": { "start_pos": 14, "end_pos": 34 },
                        "children": null
                      }
                    ]
                  },
                  {
                    "node_type": "Colon",
                    "range": { "start_pos": 34, "end_pos": 35 },
                    "children": null
                  },
                  {
                    "node_type": "IdentToken",
                    "range": { "start_pos": 35, "end_pos": 41 },
                    "children": null
                  }
                ]
              }
            ]
          },
          {
            "node_type": "DeclarationList",
            "range": { "start_pos": 42, "end_pos": 90 },
            "children": [
              {
                "node_type": "LeftCurlyBracket",
                "range": { "start_pos": 42, "end_pos": 43 },
                "children": null
              },
              {
                "node_type": "Declaration",
                "range": { "start_pos": 60, "end_pos": 76 },
                "children": [
                  {
                    "node_type": "Property",
                    "range": { "start_pos": 60, "end_pos": 67 },
                    "children": [
                      {
                        "node_type": "IdentToken",
                        "range": { "start_pos": 60, "end_pos": 67 },
                        "children": null
                      }
                    ]
                  },
                  {
                    "node_type": "Colon",
                    "range": { "start_pos": 67, "end_pos": 68 },
                    "children": null
                  },
                  {
                    "node_type": "Expression",
                    "range": { "start_pos": 69, "end_pos": 76 },
                    "children": [
                      {
                        "node_type": "Term",
                        "range": { "start_pos": 69, "end_pos": 76 },
                        "children": [
                          {
                            "node_type": "Str",
                            "range": { "start_pos": 69, "end_pos": 76 },
                            "children": null
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "node_type": "RightCurlyBracket",
                "range": { "start_pos": 89, "end_pos": 90 },
                "children": null
              }
            ]
          }
        ]
      }
    ]
  }
}
```

那么为了表示这个结构，首先我们先创建一个 ast tree 的构造器。

```rust,no_run,noplayground
{{#include ../src/ast.rs:ast_tree_builder}}
```

rust 归于树的构造和普通树的构造不一样,我们用`Option`枚举表达可以是空值，用`Box`将节点放在堆中而不是栈中。

```rust,no_run,noplayground
{{#include ../src/ast.rs:ast_tree}}
```

```rust,no_run,noplayground
{{#include ../src/ast.rs:ast_node_type}}
```

```rust,no_run,noplayground
{{#include ../src/ast.rs:ast_node}}
```

接下来我们看一下如何操作这个 ast tree 操作这个 ast 树

```rust,no_run,noplayground
{{#include ../src/ast.rs:impl}}
```

ok 最后一部就是 测试测试啦

```rust,no_run,noplayground
{{#include ../tests/test_ast_tree.rs}}
```

```sh
running 2 tests
test test_ast_tree::test_travel ... ok
test test_ast_tree::test_name ... ok
```
