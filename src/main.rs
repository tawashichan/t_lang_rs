#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(box_syntax)]

mod ast;
mod lexer;
mod parser;
mod eval;
mod interactive;
mod vm;
mod compiler;
mod object;
mod gen_x86;


use crate::ast::*;
use crate::ast::Stmt::*;
use crate::ast::Exp::*;
use crate::ast::Typ::*;
use crate::ast::Var::*;
use crate::lexer::Token;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let code = "let a = 1
    a + 1
    ";
    let code = "10 + 1";
    let tokens = lexer::str_to_tokens(code);
    let ast = parser::parse(&tokens);
    let result = gen_x86::code_gen(ast);
    write_to_file(&result);
}

fn write_to_file(s: &str) ->  Result<(), Box<std::error::Error>> {
    let mut file = File::create("code.s")?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

#[test]
fn sample_string13<'a>() {
    let s = "let a = 7 + aa(((1) + (1)),((2)),(((3)))) + 6";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![Assign(Var(format!("a")), CallFunc(format!("+"), vec![IntExp(7), CallFunc(format!("+"), vec![CallFunc(format!("aa"), vec![CallFunc(format!("+"),vec![IntExp(1),IntExp(1)]), IntExp(2), IntExp(3)]), IntExp(6)])]))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string12<'a>() {
    let s = "1 + 1";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![ExpStmt(CallFunc(format!("+"), vec![IntExp(1), IntExp(1)]))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string11<'a>() {
    let s = "fun hoge(aa Int,bb Int) Int { return 1 + 1}";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![FuncDec(format!("hoge"), vec![(format!("aa"), IntTyp),(format!("bb"), IntTyp)], IntTyp, box Block(vec![CallProc(format!("return"), vec![CallFunc(format!("+"), vec![IntExp(1), IntExp(1)])])]))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string10<'a>() {
    let s = "let a = ((7) + 4)";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![ast::Stmt::Assign(ast::Var::Var(format!("a")),Exp::CallFunc(format!("+"),vec![ast::Exp::IntExp(7),ast::Exp::IntExp(4)]))];
    let target_ast = init_prog(stmts);    
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string9<'a>() {
    let s = "let a = ((7))";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![ast::Stmt::Assign(ast::Var::Var(format!("a")),ast::Exp::IntExp(7))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string8<'a>() {
    let s = "let a = 7 + aa(1,2,3) + 6";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![Assign(Var(format!("a")), CallFunc(format!("+"), vec![IntExp(7), CallFunc(format!("+"), vec![CallFunc(format!("aa"), vec![IntExp(1), IntExp(2), IntExp(3)]), IntExp(6)])]))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string7<'a>(){
    let s = "fun huga(foo Int) Int {
        return 1 + 1
     }
     return huga(1)
    ";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![FuncDec(format!("huga"), vec![(format!("foo"), IntTyp)], IntTyp, box Block(vec![CallProc(format!("return"), vec![CallFunc(format!("+"), vec![IntExp(1), IntExp(1)])])])), CallProc(format!("return"), vec![CallFunc(format!("huga"), vec![IntExp(1)])])];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string6<'a>(){
    let s = "fun huga(foo Hoge) Int {
        return 1 + 1
     }
     return huga(1)
    ";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![FuncDec(format!("huga"), vec![(format!("foo"), NameTyp(format!("Hoge")))], IntTyp, box Block(vec![CallProc(format!("return"), vec![CallFunc(format!("+"), vec![IntExp(1), IntExp(1)])])])), CallProc(format!("return"), vec![CallFunc(format!("huga"), vec![IntExp(1)])])];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string5<'a>(){
    let s = "fun huga(foo Int) Int {
        let a = 10
        a
     }
     huga(1)
    ";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![FuncDec(format!("huga"), vec![(format!("foo"), IntTyp)], IntTyp, box Block(vec![Assign(Var(format!("a")), IntExp(10)),ExpStmt(Exp::VarExp(box Var("a".to_string())))])),ExpStmt(CallFunc(format!("huga"), vec![IntExp(1)]))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

#[test]
fn sample_string<'a>() {
    let s = "
        let hoge = 1
        fun huga(foo Int) Int {
            return foo + 1
        }
        let aaa =  4 + huga(hoge,tawawa) + 7
        print(aaa)
    ";
    let tokens = lexer::str_to_tokens(s);
    let ast = parser::parse(&tokens);
    let stmts = vec![Assign(Var(format!("hoge")), IntExp(1)), FuncDec(format!("huga"), vec![(format!("foo"), IntTyp)], IntTyp, box Block(vec![CallProc(format!("return"), vec![CallFunc(format!("+"), vec![VarExp(box Var(format!("foo"))), IntExp(1)])])])), Assign(Var(format!("aaa")), CallFunc(format!("+"), vec![IntExp(4), CallFunc(format!("+"), vec![CallFunc(format!("huga"), vec![VarExp(box Var(format!("hoge"))), VarExp(box Var(format!("tawawa")))]), IntExp(7)])])), ExpStmt(CallFunc(format!("print"), vec![VarExp(box Var(format!("aaa")))]))];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}

