# 定位

确定当前词法，语法范围，帮助后面进行错误提醒,比如以下这种。

```
  Line 3:8:  'React' is defined but never used
```

```rust,no_run,noplayground
{{#include ../src/range.rs:range}}
```

然后我们实现 Display 方法，这个方法类似于 js 的 `toString`，用于打印输出，只不过在 rust 的世界里面充分体现面向特征（类似接口）编程。

```rust,no_run,noplayground
{{#include ../src/range.rs:display}}
```

接下来我们实现一下移动范围的一些方法。

```rust,no_run,noplayground
{{#include ../src/range.rs:impl}}
```