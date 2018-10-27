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


pub fn eval_prog(p: Prog) -> Object{
    let env = Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    };
    let (ob,e) = p.stmts.into_iter().fold((Object::NoneObj,env),  |(obj,e),current|
        eval_stmt(current,e)
    );
    ob
}

fn eval_stmt(stmt: Stmt,env: Env) -> (Object,Env) {
    match stmt {
        //Stmt::CallProc(s,exps) => eval_proc(s,exps,env),
        Stmt::ExpStmt(exp) => eval_expr(exp,env),
        _ => (Object::NoneObj,env)
    }
}

fn eval_expr(exp: Exp,env: Env) -> (Object,Env) {
    match exp {
        //Exp::BoolExp(b) => (Object::Bool(*b),env)
        Exp::IntExp(i) => (Object::Int(i),env),
        //Exp::CallFunc(name,exps) => exec_func(name,exps,env),
        _ => (Object::Int(0),env)
    }
}

/*pub fn eval_proc(s: String,exps: Vec<Exp>,env: Env) -> Env {
    /*exps.into_iter().fold(env,|e,current|
        eval_expr(current,&e)
    );*/
    env
}*/

fn exec_func(name: String,exps: Vec<Exp>,env: Env) -> Object {
    match name.as_str() {
        "print" => {
            check_arg_num(&name,&exps);
            let obj = eval_expr(exps.first().unwrap().clone(),env);
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
    let prog = Prog{stmts: vec![Stmt::ExpStmt(Exp::IntExp(10))]};
    let obj = eval_prog(prog);
    assert_eq!(obj,Object::Int(10))
}

/*#[test]
fn check_func2(){
     let env = Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    };
    let exp = Exp::CallFunc(format!("print"),vec![Exp::IntExp(10)]);
    let (obj,e) = eval_expr(exp,env);
    assert_eq!(obj,Object::NoneObj)
}*/