#![feature(test)]
extern crate test;
extern crate kernel;

use test::Bencher;
use kernel::commands::*;
use kernel::commands::ast::*;
use kernel::streams::interpreter::*;

#[bench]
fn empty(b: &mut Bencher) {
    b.iter(|| 1)
}

#[bench]
fn parse1(b: &mut Bencher) {
    let mut i = Interpreter::new().unwrap();
    b.iter(|| {
        i.parse(&"1*2+3".to_string());
    })
}

#[bench]
fn parse2(b: &mut Bencher) {
    let mut i = Interpreter::new().unwrap();
    b.iter(|| {
        i.parse(&"+/{x*y}[(a;b;c;d;e);(2;6;2;1;3)]".to_string());
    })
}

// #[bench]
// #fn k_plus(b: &mut Bencher) {
// #b.iter(|| ast::eval(AST::Verb(Verb::Plus, AST::Number(2).boxed(), AST::Number(3).boxed())));
//

#[bench]
fn parse4(b: &mut Bencher) {
    let mut i = Interpreter::new().unwrap();
    b.iter(|| {
        i.parse(&"();[];{};(());[[]];{{}};()();1 2 3;(1 2 3);[1 2 \
                            3];[a[b[c[d]]]];(a(b(c(d))));{a{b{c{d}}}};"
            .to_string());
    })
}

#[bench]
fn fac_rust(b: &mut Bencher) {
    let mut x: i64 = 0;
    let mut a: i64 = 5;
    b.iter(|| {
        x = factorial(a);
    });
}

#[inline]
fn factorial(value: i64) -> i64 {
    if value == 1 {
        1
    } else {
        return value * factorial(value - 1);
    }
}

#[bench]
fn fac(b: &mut Bencher) {
    let mut i = Interpreter::new().unwrap();
    let code = i.parse(&"fac:{$[x=1;1;x*fac[x-1]]}".to_string());
    i.run(code).unwrap();
    let f = i.parse(&"fac[5]".to_string());
    b.iter(|| {
        i.run(f);
        i.gc();
    })
}
