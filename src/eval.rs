///意味解析フェーズ
/// 型検査と環境の設定を行う

use ast::{Typ,Exp,Stmt,Dec};
use std::collections::HashMap;

pub struct FuncContent {
    name: String,
    args: Vec<(String,Typ)>,
    content: Vec<Stmt>,
    return_type: Typ
}

pub enum Val {
    Int(i64),
    String(String)
}

//関数や変数を管理
pub struct Env{
    vars: HashMap<String,Exp>,
    functions: HashMap<String,FuncContent>,
    prev: Option<Box<Env>>
}



/*pub fn evaluate_program(p: Program) -> Env {

    let env = Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: None
    };
    p.nodes.into_iter().fold(env,  |e,current|
        evaluate_node(current,e)
    )
}

fn evaluate_node(n: Node,env: Env) -> Env {
    return Env{
        vars: HashMap::new(),
        functions: HashMap::new(),
        prev: Some(box env)
    }
}*/

/*fn check_node(n: Node,env: Env) -> env {
    match n {
        Node::Statement(stmt) => match stmt {
            Statement::Def(def) => match def {
                Def::VarDef {name,value} => {
                    env.vars.insert(name,value);
                    env
                }
            }
        }
    }
}

fn evaluate_expr(expr: Expr) -> Val {

}*/



