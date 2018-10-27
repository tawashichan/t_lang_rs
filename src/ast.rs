/// 文法(どっかでちゃんとBNFでかきたい)
///
/// 変数宣言
/// let hoge = 1
///
/// 関数
/// function hoge (huga Type,foo Type) Type {
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
    VarDec(Typ,String),
    StructDec(String,HashMap<String,Typ>),
    ExpStmt(Exp), // 式文
    NilStmt
}

#[derive(Debug,Clone,PartialEq)]
pub enum Exp {
    VarExp(Box<Var>),
    StrExp(String),
    IntExp(i64),
    BoolExp(bool),
    CallFunc(String,Vec<Exp>),
    If(Box<Exp>,Box<Stmt>,Option<Box<Stmt>>),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Typ {
    NameTyp(String),
    ArrayTyp(i64,Box<Typ>),
    StrTyp,
    IntTyp,
    VoidTyp,
}


