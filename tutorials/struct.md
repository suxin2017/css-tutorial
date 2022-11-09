# 结构体、入口函数

上篇我们实现了范围类型，Token

这边我们直接进入正题，开始实现词法解析。

首先是最为简单的符号，那么这些符号我们去哪找呢？

答案就是草案，发明语法实际上远远比实现语法解析要难得多，前者可以比作图纸，后者比做建造。

[consume-token](https://www.w3.org/TR/css-syntax-3/#consume-token)

接下来就是介绍一下词法解析起的运作流程了

```
由语法分析进行调用
获取token -> 是否匹配 --> 消费token
               |
               V
            抛出异常
```

首先我们实现 lexer 的结构体

```rust,no_run,noplayground
{{#include ../src/lexer.rs:lexer}}
```

这里有一个 rust 的新手常见的坑，或者说迷惑的地方，就是 String str &str 的关系，老实讲知道现在有时候我也很模糊。

我的理解如下

- str 是静态字符串，编译器存储在二进制文件中。
- String 属于动态字符串，存储在堆中。
- &str 是切片，或者说指针更好一些，他只是一个指向内存地址的指针

```rust
# fn main() {
     let x = "i am str";
    // "i am str" 这是str 存储在二进制中
    // x 是指向 str 的指针，所以他的类型是 &str
    let y = String::from("string");
    // 转化 y 到 &str
    let z = &y[..];
    //  z &str
    // z 是指向y的指针
# }
```

这里的 chars 是 source_code 的切片，方便我们进行字符遍历。

接下来我们实现 lexer 的入口函数

```rust,no_run,noplayground
{{#include ../src/lexer.rs:get_token}}
```
