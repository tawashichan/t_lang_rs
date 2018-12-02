/// 文法(どっかでちゃんとBNFでかきたい)
///
/// 変数宣言
/// let hoge = 1
///
/// 関数
/// fun hoge (huga Type,foo Type) Type {
///
/// }
///
/// let hoge = 1;
///
/// function huga(foo Int) Int {
///     foo + 1
/// }
///
/// let aaa == huga(hoge)
///
///
///
///

use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub struct Prog{
    pub stmts: Vec<Stmt>
}

#[derive(Debug,Clone,PartialEq)]
pub enum Var {
    Var(String),
    IndexedVar(Box<Var>,Exp)
}

#[derive(Debug,Clone,PartialEq)]
pub enum Stmt {
    Assign(Var,Exp),
    CallProc(String,Vec<Exp>),
    Block(Vec<Stmt>),
    FuncDec(String,Vec<(String,Typ)>,Typ,Box<Stmt>),
    StructDec(String,HashMap<String,Typ>),
    ExpStmt(Exp), // 式文
    Return(Exp),
    NilStmt
}

#[derive(Debug,Clone,PartialEq)]
pub enum Exp {
    VarExp(Box<Var>),
    StrExp(String),
    IntExp(i64),
    BoolExp(bool),
    ArrayExp(Vec<Exp>),
    CallFunc(String,Vec<Exp>),
    If(Box<Exp>,Box<Stmt>,Box<Option<Stmt>>),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Typ {
    NameTyp(String),
    ArrayTyp(i64,Box<Typ>),
    BoolTyp,
    StrTyp,
    IntTyp,
    VoidTyp,
}


pub fn init_prog(stmts: Vec<Stmt>) -> Prog {
    Prog {
        stmts
    }
} 

