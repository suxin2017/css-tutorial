# 准备环境

## 安装 rust 环境

[rustup](https://rustup.rs/#)

检测你的安装

```bash
rustc --version
```

```
cargo --version
```

## 创建你的项目

```
cargo init my-css
```

我们使用 `cargo` 创建了一个可以运行二进制的 `bin` 项目。

`cargo` 可以创建了 2 中类型的 cargo 包，一种是 `bin` 类型的，一种是 `lib` 类型的。

`lib` 类型的是提供给别人使用的
`bin` 类型的是可以通过命令行使用的

当然一个 `cargo` 的包可以即使 `bin` 类型的也是 `lib` 类型的。
你的项目目录应该是以下这样的

```bash
cd my-css
tree .
```

```
my-css
├── Cargo.toml
└── src
    └── main.rs
```

## 创建测试目录

我们在开发过程中需要不断的进行测试，所以我们在项目下面创建 `tests`目录。

作为初学者我们最好在一开始应该遵循比较好的最佳实践，所以我们的目录结构按照这个目录进行 [Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)

```rust,ignore
// tests/main.rs
#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
```

运行

```bash
cargo test
```

## 下一步
未来我们将围绕 css 语法草案进行我们的编写，所以我们可以提前熟悉下css完整的语法。
[CSS Syntax Module Level 3](https://www.w3.org/TR/css-syntax-3/)