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

#[derive(Clone,Debug,PartialEq)]
pub enum Object {
    Int(i64),
    String(String),
    Bool(bool),
    NoneObj,
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
    /*match stmt {
        Stmt::CallProc(s,exps) => eval_proc(s,exps,env),
        Stmt::ExpStmt(exp) => eval_expr(exp,env),
        _ => env
    }*/
    env
}

pub fn eval_expr(exp: &Exp,env: &Env) -> Object {
    match exp {
        Exp::BoolExp(b) => Object::Bool(*b),
        Exp::IntExp(i) => Object::Int(*i),
        Exp::CallFunc(name,exps) => exec_func(name,exps,env),
        _ => Object::Int(0)
    }
}

pub fn eval_proc(s: String,exps: Vec<Exp>,env: Env) -> Env {
    /*exps.into_iter().fold(env,|e,current|
        eval_expr(current,&e)
    );*/
    env
}

fn exec_func(name: &String,exps: &Vec<Exp>,env: &Env) -> Object {
    match name.as_str() {
        "print" => {
            check_arg_num(&name,&exps);
            let obj = eval_expr(exps.first().unwrap(),&env);
            println!("{:?}",obj);
            Object::NoneObj
        }
        _ => {
            Object::NoneObj
        }    
    }
}

fn check_arg_num(func_name: &str,exps: &Vec<Exp>) {
    match func_name {
        "print" => {
            if exps.len() != 1 {
                panic!("too many args for print")
            }
        }
        _ => {}
    }
}

#[test]
fn check_func(){
     let env = Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    };
    let exp = Exp::IntExp(10);
    let obj = eval_expr(&exp,&env);
    assert_eq!(obj,Object::Int(10))
}

#[test]
fn check_func2(){
     let env = Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    };
    let exp = Exp::CallFunc(format!("print"),vec![Exp::IntExp(10)]);
    let obj = eval_expr(&exp,&env);
    assert_eq!(obj,Object::NoneObj)
}