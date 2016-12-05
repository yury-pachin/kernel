// An add verb combinator.

use streams::interpreter::*;
use commands::ast::AST;
use streams::stream::{self, Error, Poll, Async};

pub struct Eq {
    lvalue: AST,
    rvalue: AST,
}

pub fn new(lvalue: AST, rvalue: AST) -> Eq {
    Eq {
        lvalue: lvalue,
        rvalue: rvalue,
    }
}

impl Eq {
    // Now just returning simple int
    fn a_a(l: i64, r: i64) -> AST {
        // Need to figure out what integers we have (signed or unsigned)
        AST::Number(if r == l { 1 } else { 0 })
    }
    fn l_a(l: AST, r: AST) -> AST {
        AST::Number(1)
    }
    fn a_l(l: AST, r: AST) -> AST {
        AST::Number(1)
    }
    fn l_l(l: &[i64], r: &[i64]) -> AST {
        AST::Number(1)
    }
}

impl Iterator for Eq {
    type Item = AST;
    fn next(&mut self) -> Option<Self::Item> {
        match (&mut self.lvalue, &mut self.rvalue) {
            (&mut AST::Number(ref l), &mut AST::Number(ref r)) => Some(Self::a_a(*l, *r)),
            _ => None,
        }
    }
}

impl<'a> Iterator for &'a Eq {
    type Item = AST;

    fn next(&mut self) -> Option<Self::Item> {
        // not implemented yet
        None
    }
}
