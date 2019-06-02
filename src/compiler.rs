/// 抽象構文器を辿ってVM向けのバイトコードを生成する

use crate::ast::{Exp,Prog,Stmt};
use crate::vm::{Instruction};
use crate::object::{Object};

pub struct State {
    instructions: Vec<Instruction>,
    constants: Vec<Object>
}

impl State {

    fn new() -> State {
        State{
            instructions: vec![],
            constants: vec![]
        }
    }

    fn add_constant(&mut self,obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

}

pub fn compile(p: Prog) -> State {
    let state = State::new();
    p.stmts.into_iter().fold(state,|s,stmt|
        process_stmt(s,stmt)
    )
}

fn process_stmt(state: State,stmt: Stmt) -> State {
    match stmt {
        Stmt::ExpStmt(exp) => process_exp(state,&exp),
        _ => State::new()
    }
}

fn process_exp(mut state: State,exp: &Exp) -> State {
    match exp {
        Exp::IntExp(i) => {
            let int_index = state.add_constant(Object::Int(i.clone()));
            state
        }
        //Exp::CallFunc(f_name,exps) => process_call_func(state,f_name,exps),
        _ => State::new()
    }
}

/*fn process_call_func(state: State,name: &String,exps: &Vec<Exp>) -> State {
    match name.as_ref() {
        "+"  => {
            let e1 = process_exp(&exps[0]);
            let e2 = process_exp(&exps[1]);
            State::new()
        }
        _ => State::new()
    }
}*/

#[test]
fn add_constant1(){
    let mut state = State::new();
    state.add_constant(Object::Int(0));
}



