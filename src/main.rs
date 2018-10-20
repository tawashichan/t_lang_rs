#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(nll)]

mod ast;
mod lexer;
mod parser;
mod eval;

use ast::*;
use ast::Stmt::*;
use ast::Exp::*;

fn main() {
    let s = "let a = (((7)) + 4)";
    let tokens = lexer::str_to_tokens(s);
    println!("{:?}",s);
    println!("{:?}",parser::parse(&tokens))
}

#[test]
fn sample_string9<'a>() {
    let s = "let a = (7)";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let target_ast = ast::Prog{
        stmts: vec![ast::Stmt::Assign(ast::Var::Var(format!("a")),ast::Exp::IntExp(7))]
    };
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string8<'a>() {
    let s = "let a = 7 + aa(1,2,3) + 6";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let target_ast = Prog {
        stmts: vec![Assign(Var::Var(format!("a")), CallFunc(format!("+"), vec![IntExp(7), CallFunc(format!("+"), vec![CallFunc(format!("aa"), vec![IntExp(1), IntExp(2), IntExp(3)]), IntExp(6)])]))]
    };
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string7<'a>(){
    let s = "function huga(foo Int) Int {
        return 1 + 1
     }
     return huga(1)
    ";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    println!("{:?}",parser::parse(&tokens))
}

#[test]
fn sample_string<'a>() {
    let s = "
        let hoge = 1
        function huga(foo Int) Int {
            return foo + 1
        }
        let aaa =  4 + huga(hoge,tawawa) + 7
        print(aaa)
    ";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    println!("{:?}", parser::parse(&tokens))
}

fn sample_string6<'a>() -> &'a str {
    "let a = 1 + 1 + 6"
}

fn sample_string5<'a>() -> &'a str {
    "return 1 + 1"
}

fn sample_string4<'a>() -> &'a str {
    "let fa = 1 + 1
     let aa = 2 + 2
    "
}

fn sample_string3<'a>() -> &'a str {
    "function huga(foo Int) Int {
            foo + 1
      }
    "
}

fn sample_string2<'a>() -> &'a str {
    "let hoge = 1
     let aaa = huga(hoge)
    "

}




