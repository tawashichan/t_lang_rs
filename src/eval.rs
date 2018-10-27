///意味解析フェーズ
/// 型検査と環境の設定を行う

use ast::{Typ,Exp,Stmt,Prog};
use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct FuncContent {
    name: String,
    args: Vec<(String,Typ)>,
    content: Vec<Stmt>,
    return_type: Typ
}

#[derive(Clone,Debug)]
pub enum Val {
    Int(i64),
    String(String)
}

//関数や変数を管理
#[derive(Clone,Debug)]
pub struct Env{
    vars: HashMap<String,Exp>,
    functions: HashMap<String,FuncContent>,
    prev: Option<Box<Env>>
}


pub fn eval_prog(p: Prog) -> Env{
    let env = Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    };
    p.stmts.into_iter().fold(env,  |e,current|
        eval_stmt(current,e)
    )
}

pub fn eval_stmt(stmt: Stmt,env: Env) -> Env {
    match stmt {
        Stmt::CallProc(s,exps) => eval_proc(s,exps,env),
        Stmt::ExpStmt(exp) => eval_expr(exp,env),
        _ => env
    }
}

pub fn eval_expr(exp: Exp,env: Env) -> Env {
    match exp {
        Exp::BoolExp(b) => env,
        _ => env
    }
}

pub fn eval_proc(s: String,exps: Vec<Exp>,env: Env) -> Env {
    exps.into_iter().fold(env,|e,current|
        eval_expr(current,e)
    )
}


