# 词法 Token 定义

首先我们来定义一下词法 token。

那么词法 token 哪里来呢？

从这篇文章中我们可以找到对于词法 token 的定义。

[Appendix G. Grammar of CSS 2.2](https://www.w3.org/TR/CSS22/grammar.html)

```rust,no_run,noplayground
// src/token.rs

{{#include ../src/token.rs:token}}
```

我们用元组结构体来表达 `Token`，它包含一个 `TokenType` 和一个范围 `Range`;

并且我们为 `Token` 实现了两个方法，一个是检测当前 `Token` 是否是传入的类型。

一个是通过原始字符串，获取当前 `Token` 字符。

接下来我们为我们第一个比较复杂的结构体创建测试 case;

```rust,no_run,noplayground
// tests/test_token.rs

{{#include ../tests/test_token.rs}}
```

```bash
# 运行测试
cargo run test test_token

# output
running 2 tests
test test_token::check_token_type ... ok
test test_token::get_source_code ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

好了我们的方法符合预期的运行，与业务开发不同，这个教程会编写大部分测试 case，来保证最后能够在真实场景使用。

## TokenType 定义

接下来我们使用一个枚举来定义我们这次使用的所有`Token` 类型。

```rust,no_run,noplayground
// src/token_type.rs

{{#include ../src/token_type.rs:lexer_token_type}}
}
```

这部分比较简单就不一一介绍了。

接下来就是我们的词法解析器的实现了。
