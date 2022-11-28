# 定位

确定当前词法，语法范围，帮助后面进行错误提醒,比如以下这种。

```
  Line 3:8:  'React' is defined but never used
```

```rust,no_run,noplayground
{{#include ../src/range.rs:range}}
```

这里通过 `#[dirvie]` 进行包裹的是 rust 的指令宏，这里简单介绍下 rust 的宏。

```rust
macro_rules! say_hello {
    (console.log("hello world")) => {
        println!("hello world");
    };
}

fn main(){
  say_hello!(console.log("hello world"));
}

```

点击左上角的运行按钮，你会发现这代码竟然能运行，我很简单的就实现了一个一个 js 解释器。

他的魔法在于在编译期将你的代码进行了替换，可以在 rust playground 中看下他的宏展开。

你会发现你的代码被替换成下面这种，这种编译器做的拦截对运行时毫无影响。

```rust,no_run,noplayground
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
macro_rules! say_hello {
    (console.log("hello world")) => { println! ("hello world") ; } ;
}

fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["hello world\n"],
                &[]));
    };
    ;
}
```

相比于 js 的拦截器的实现高出很多。有点类似 svelte 的原理。

当然 rust 的宏并不是完美的，它带来的代价就是增加包体积，代码编译慢。

接下来，为`Range`添加 new 的创建方法

```rust,no_run,noplayground
{{#include ../src/range.rs:impl}}
```

好啦本节就到此结束啦。

后面我们开始介绍词法解析器部分。
