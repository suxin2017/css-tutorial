# 词法解析器开发

语法解析器相对于词法解析器更加简单，他的困难在于语法定义，什么是语法定义呢？

一般通过 bnf 或 ebnf 来描述,不过 bnf 和 ebnf 并没有相对的规范，不过其表达的意义一般都差不多。

> 扩展巴科斯-瑙尔范式（EBNF, Extended Backus–Naur Form）是表达作为描述计算机编程语言和形式语言的正规方式的上下文无关文法的元语法(metalanguage)符号表示法。它是基本巴科斯范式(BNF)元语法符号表示法的一种扩展。

有兴趣的读者可以看这篇文章。

[wiki ebnf ](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)

css 的 ebnf

[Appendix G. Grammar of CSS 2.2](https://www.w3.org/TR/CSS22/grammar.html)

```ebnf
stylesheet
  : [ CHARSET_SYM STRING ';' ]?
    [S|CDO|CDC]* [ import [ CDO S* | CDC S* ]* ]*
    [ [ ruleset | media | page ] [ CDO S* | CDC S* ]* ]*
  ;
import
  : IMPORT_SYM S*
    [STRING|URI] S* media_list? ';' S*
  ;
media
  : MEDIA_SYM S* media_list '{' S* ruleset* '}' S*
  ;
media_list
  : medium [ COMMA S* medium]*
  ;
medium
  : IDENT S*
  ;
page
  : PAGE_SYM S* pseudo_page?
    '{' S* declaration? [ ';' S* declaration? ]* '}' S*
  ;
pseudo_page
  : ':' IDENT S*
  ;
operator
  : '/' S* | ',' S*
  ;
combinator
  : '+' S*
  | '>' S*
  ;
property
  : IDENT S*
  ;
ruleset
  : selector [ ',' S* selector ]*
    '{' S* declaration? [ ';' S* declaration? ]* '}' S*
  ;
selector
  : simple_selector [ combinator selector | S+ [ combinator? selector ]? ]?
  ;
simple_selector
  : element_name [ HASH | class | attrib | pseudo ]*
  | [ HASH | class | attrib | pseudo ]+
  ;
class
  : '.' IDENT
  ;
element_name
  : IDENT | '*'
  ;
attrib
  : '[' S* IDENT S* [ [ '=' | INCLUDES | DASHMATCH ] S*
    [ IDENT | STRING ] S* ]? ']'
  ;
pseudo
  : ':' [ IDENT | FUNCTION S* [IDENT S*]? ')' ]
  ;
declaration
  : property ':' S* expr prio?
  ;
prio
  : IMPORTANT_SYM S*
  ;
expr
  : term [ operator? term ]*
  ;
term
  : [ NUMBER S* | PERCENTAGE S* | LENGTH S* | EMS S* | EXS S* | ANGLE S* |
      TIME S* | FREQ S* ]
  | STRING S* | IDENT S* | URI S* | hexcolor | function
  ;
function
  : FUNCTION S* expr ')' S*
  ;
/*
 * There is a constraint on the color that it must
 * have either 3 or 6 hex-digits (i.e., [0-9a-fA-F])
 * after the "#"; e.g., "#000" is OK, but "#abcd" is not.
 */
hexcolor
  : HASH S*
  ;
```

首先是定义 parse 结构

```rust,no_run,noplayground
{{#include ../src/parser.rs:parser}}
```

这里我们因为借用词法解析器，和 ast 构建器。

并且标记他们的生命周期`'a`

为了方便使用我们编写一些包裹 lexer 的方法。

```rust,no_run,noplayground
{{#include ../src/parser.rs:lexer_wrapper}}
```

接下来是 parser 的入口函数

```rust,no_run,noplayground
{{#include ../src/parser.rs:entry}}
```

这里对应的便是

```
stylesheet
  : [ CHARSET_SYM STRING ';' ]?
    [S|CDO|CDC]* [ import [ CDO S* | CDC S* ]* ]*
    [ [ ruleset | media | page ] [ CDO S* | CDC S* ]* ]*
  ;
```

通过我们之前写的 ast tree 的代码，我们可以看出，我们只是对 ast 进行实现。

后面的也是一样的就不做过多介绍了。

最后还是测试测试

```rust,no_run,noplayground
{{#include ../tests/test_parser.rs:exapmle}}
```

```
test tests::simple7_test ... ok
```
