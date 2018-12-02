/// とりあえずインタプリタ
/// 型検査と環境の設定を行う

use ast::{Typ,Exp,Stmt,Prog,Var,init_prog};
use std::collections::HashMap;

#[derive(Clone,Debug)]
pub struct FuncContent {
    args: Vec<(String,Typ)>,
    content: Stmt,
    return_type: Typ
}

#[derive(Clone,Debug,PartialEq)]
pub enum Object {
    Int(i64),
    String(String),
    Bool(bool),
    Struct(String,HashMap<String,Object>),
    NoneObj,
}

//関数や変数を管理
//コピー発生しまくりなのでどっかで参照になおす
#[derive(Clone,Debug)]
pub struct Env{
    vars: HashMap<String,Object>,
    functions: HashMap<String,FuncContent>,
    prev: Option<Box<Env>>
}


pub fn eval_prog(p: Prog,env: Env) -> (Object,Env) {
    p.stmts.into_iter().fold((Object::NoneObj,env),  |(obj,e),current|
        eval_stmt(current,e)
    )
}

pub fn init_env() -> Env {
     Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    }
} 

fn make_local_env(env: Env) -> Env {
    Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: Some(box env)
    }
}

fn eval_stmt(stmt: Stmt,mut env: Env) -> (Object,Env) {
    match stmt {
        Stmt::CallProc(s,exps) => (eval_proc(s,exps,&env),env),
        Stmt::Block(stmts) => {
            let local_env = make_local_env(env.clone());
            let (obj,e) = stmts.into_iter().fold((Object::NoneObj,local_env),  |(obj,e),current|
                eval_stmt(current,e)
            );
            (obj,env)
        } 
        Stmt::FuncDec(name,args,return_type,box content) => {
            let func = FuncContent{
                args: args,
                return_type: return_type,
                content: content
            };
            env.functions.insert(name.clone(),func);
            (Object::NoneObj,env)
        }
        Stmt::Assign(Var::Var(s),exp) =>  {
            let obj = eval_exp(exp,&env);
            env.vars.insert(s,obj);
            (Object::NoneObj,env)
        }
        Stmt::ExpStmt(exp) => (eval_exp(exp,&env),env),
        _ => (Object::NoneObj,env)
    }
}

fn eval_block(block: Stmt,env: &Env) -> Object {
    match block {
        Stmt::Block(stmts) =>
            match &stmts[..] {
                [Stmt::Return(exp),rest..] => {
                    eval_exp(exp.clone(), env)
                },
                [last] => {
                    let (obj,_) = eval_stmt(last.clone(), env.clone());
                    obj
                },
                _ => Object::NoneObj
            }
        _ => panic!("{:?}",block)
    }
}

fn eval_exp(exp: Exp,env: &Env) -> Object {
    match exp {
        Exp::BoolExp(b) => Object::Bool(b),
        Exp::IntExp(i) => Object::Int(i),
        Exp::CallFunc(name,exps) => call_func(name,exps,env),
        Exp::VarExp(box Var::Var(s)) => {
            let val = search_val(s.clone(),&env); 
            match val {
                Some(v) => {
                    v
                }
                None => {
                    panic!("val {:?} is undefined",s.clone())
                }
            }
        }
        _ => Object::Int(0)
    }
}

fn eval_proc(name: String,exps: Vec<Exp>,env: &Env) -> Object {
    call_func(name,exps,env)
}

 
fn call_func(name: String,exps: Vec<Exp>,env: &Env) -> Object {
    match name.as_str() {
        "print" => {
            check_arg_num(&name,&exps);
            let obj = eval_exp(exps.first().unwrap().clone(),env);
            println!("{:?}",obj);
            Object::NoneObj
        }
        "+" => {
            check_arg_num(&name,&exps);
            let arg1 = eval_exp(exps[0].clone(),env);
            let arg2 = eval_exp(exps[1].clone(),env);
            add_int(arg1,arg2)
        }
        "-" => {
            check_arg_num(&name,&exps);
            let arg1 = eval_exp(exps[0].clone(),env);
            let arg2 = eval_exp(exps[1].clone(),env);
            minus_int(arg1,arg2)
        }
        "*" => {
            check_arg_num(&name,&exps);
            let arg1 = eval_exp(exps[0].clone(),env);
            let arg2 = eval_exp(exps[1].clone(),env);
            mul_int(arg1,arg2)
        }
        "/" => {
            check_arg_num(&name,&exps);
            let arg1 = eval_exp(exps[0].clone(),env);
            let arg2 = eval_exp(exps[1].clone(),env);
            div_int(arg1,arg2)
        }
        _ => {
            call_decleared_func(name, exps, env)
        }    
    }
}

fn call_decleared_func(name: String,args: Vec<Exp>,env: &Env) -> Object {
    let func = search_func(name,&env).expect("no such function");
    let mut local_env = make_local_env(env.clone());
    let def_args_len = func.args.len();
    let call_args_len = args.len();
    if def_args_len != call_args_len {
        panic!("invalid length of arguments: def {:?} call {:?}",func.args,args)
    }
    let e = bind_args(func.args, args, &mut local_env);
    eval_block(func.content, e)
}

fn bind_args(def_args: Vec<(String,Typ)>,call_args: Vec<Exp>,env: &mut Env) -> &Env {
    let mut objs = vec![];
    for exp in call_args.into_iter() {
        let obj = eval_exp(exp, &env);
        objs.push(obj);
    }
    for ((n,typ),c) in def_args.into_iter().zip(objs.into_iter()) {
        check_type(&c,&typ);
        env.vars.insert(n,c);
    }
    env
}


fn add_int(i1: Object,i2: Object) -> Object {
    match i1 {
        Object::Int(i) => {
            match i2 {
                Object::Int(ii) => {
                    Object::Int(i + ii)
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn mul_int(i1: Object,i2: Object) -> Object {
    match i1 {
        Object::Int(i) => {
            match i2 {
                Object::Int(ii) => {
                    Object::Int(i * ii)
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn minus_int(i1: Object,i2: Object) -> Object {
    match i1 {
        Object::Int(i) => {
            match i2 {
                Object::Int(ii) => {
                    Object::Int(i - ii)
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn div_int(i1: Object,i2: Object) -> Object {
    match i1 {
        Object::Int(i) => {
            match i2 {
                Object::Int(ii) => {
                    Object::Int(i / ii)
                }
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn check_arg_num(func_name: &str,exps: &Vec<Exp>) {
    match func_name {
        "print" => {
            if exps.len() != 1 {
                panic!("too many args for print")
            }
        }
        "+" | "-" | "*" | "/" => {
            if exps.len() != 2 {
                panic!("too many args for print")
            }
        }
        _ => {}
    }
}

fn search_func(name: String,env: &Env) -> Option<FuncContent> {
    match env.functions.get(&name) {
        Some(f) => Some(f.clone()),
        None => match &env.prev {
            Some(p) => search_func(name,p),
            None => None
        }
    }
}

fn search_val(name: String,env: &Env) -> Option<Object> {
    match env.vars.get(&name) {
        Some(v) => Some(v.clone()),
        None => match &env.prev {
            Some(p) => search_val(name,p),
            None => None
        }
    }
}

fn check_type(target: &Object,expected: &Typ) {
    match target {
        Object::Int(i) => match expected {
            Typ::IntTyp => {},
            _ => panic!("invalid type: expected: Int actual: {:?}",target)
        }
        _ => panic!("invalid type: expected: Int actual: {:?}",target)
    }
}


#[test]
fn check_func(){
    let prog = init_prog(vec![Stmt::ExpStmt(Exp::IntExp(10))]);
    let (obj,_e) = eval_prog(prog,init_env());
    assert_eq!(obj,Object::Int(10))
}

#[test]
fn check_assign(){
    let stmt = Stmt::Assign(Var::Var(format!("hoge")),Exp::IntExp(10));
    let (obj,e) = eval_stmt(stmt,init_env());
    println!("{:?}",e);
    assert_eq!(obj,Object::NoneObj)
}

#[test]
fn eval_block1(){
    let prog = init_prog(vec![Stmt::ExpStmt(Exp::IntExp(10))]);
    let (obj,_e) = eval_prog(prog,init_env());
    assert_eq!(obj,Object::Int(10))
}


/*#[test]
fn check_var(){
    let prog = Prog{stmts: vec![Stmt::Assign(Var::Var(format!("hoge")),Exp::IntExp(10)),Stmt::]};
    let obj = eval_prog(prog);
    assert_eq!(obj,Object::NoneObj)
}*/