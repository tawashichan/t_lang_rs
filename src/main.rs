#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(box_syntax)]

mod ast;
mod lexer;
mod parser;
mod eval;

use ast::{Node,Statement,Def,Type,Val,TwoTermOp,Program,Expr};

fn main() {
    let n = Node::Statement(Statement::Def(Def::VarDef{name: "hoge".to_string(),value: Expr::Val(Val::Int(1))}));
    let n2 = Node::Statement(Statement::Def(Def::FuncDef{name: "huga".to_string(),args: vec![("foo".to_string(),Type::Int)],content: vec![Node::Expr(Expr::TwoTermOp(box TwoTermOp::Plus(Expr::Var("foo".to_string()),Expr::Val(Val::Int(1)))))],return_type: Type::Int}));
    let n3 =  Node::Statement(Statement::Def(Def::VarDef{name: "aaa".to_string(),value: Expr::FuncApply(format!("huga"),vec![Expr::Var(format!("hoge"))])}));

    let p = Program{nodes: vec![n,n2,n3]};

    println!("{:?}",p);



}
