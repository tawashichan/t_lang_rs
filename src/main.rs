#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(nll)]

mod ast;
mod lexer;
mod parser;
mod eval;
mod interactive;
mod proc_parser;


use ast::*;
use ast::Stmt::*;
use ast::Exp::*;
use ast::Typ::*;
use ast::Var::*;
use lexer::Token;

fn main() {
    interactive::start_itl();
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
    let stmts = vec![Assign(Var(format!("hoge")), IntExp(1)), FuncDec(format!("huga"), vec![(format!("foo"), IntTyp)], IntTyp, box Block(vec![CallProc(format!("return"), vec![CallFunc(format!("+"), vec![VarExp(box Var(format!("foo"))), IntExp(1)])])])), Assign(Var(format!("aaa")), CallFunc(format!("+"), vec![IntExp(4), CallFunc(format!("+"), vec![CallFunc(format!("huga"), vec![VarExp(box Var(format!("hoge"))), VarExp(box Var(format!("tawawa")))]), IntExp(7)])])), CallProc(format!("print"), vec![VarExp(box Var(format!("aaa")))])];
    let target_ast = init_prog(stmts);
    assert_eq!(target_ast,ast)
}


