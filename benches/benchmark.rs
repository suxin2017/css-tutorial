#![allow(unused)]

use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use css_tutorial::{ast::AstTreeBuilder, lexer::Lexer, parser::Parser, token_type::TokenType};
use rand::Rng;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

macro_rules! test_token {
    ($x:expr) => {
        let mut lexer = Lexer::new($x);
        loop {
            let node = lexer.eat_token();
            if node.check_type(TokenType::EOF) {
                break;
            }
        }
    };
}

fn gener_random_ident() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                           _";
    const PASSWORD_LEN: usize = 10;
    let mut rng = rand::thread_rng();

    let mut str = vec![];
    for _ in 0..=1000 {
        let result: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        str.push(result);
    }
    return str.join(" ");
}

fn gener_random_number() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"123456789";
    const PASSWORD_LEN: usize = 10;
    let mut rng = rand::thread_rng();

    let mut str = vec![];
    for _ in 0..=1000 {
        let result: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        str.push(result);
    }
    return str.join(" ");
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse 800 line benchmark", |b| {
        let binding = fs::read_to_string("test.css").unwrap();

        b.iter(|| {
            let mut lexer = Lexer::new(&binding);
            let mut builder = AstTreeBuilder::new();
            let mut parser = Parser::new(&mut lexer, &mut builder);
            parser.parse();
        })
    });
    c.bench_function("lexer 800 line benchmark", |b| {
        b.iter(|| {
            let binding = fs::read_to_string("test.css").unwrap();
            let mut lexer = Lexer::new(&binding);
            loop {
                let node = lexer.eat_token();
                if node.check_type(TokenType::EOF) {
                    break;
                }
            }
        })
    });
}

criterion_group!( name = benches;
  // This can be any expression that returns a `Criterion` object.
  config = Criterion::default().sample_size(10);
  targets = criterion_benchmark
);
criterion_main!(benches);
