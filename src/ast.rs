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
/// 上の構文は
///  Node::Statement(Statement::Def(Def::VarDef{name: "hoge".to_string(),value: Expr::Int(1)}));
///  Node::Statement(Statement::Def(Def::FuncDef{name: "huga".to_string(),args: vec![("foo".to_string(),Type::Int)],content: vec![Node::Expr(Expr::TwoTermOp(format!("PLUS"),box Expr::Var("foo".to_string()),box Expr::Int(1)))],return_type: Type::Int}));
///  Node::Statement(Statement::Def(Def::VarDef{name: "aaa".to_string(),var_type: Type::Int,value: Expr::FuncApply(format!("huga"),vec![Expr::Var(format!("hoge"))])}));
/// となる予定。とりあえずこれをコンパイルするのを目標にする。改装深すぎるのでtraitでいい感じにできればなぁ
///
///
///
///

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
    //Block(Vec<Dec>,Vec<Stmt>),
    FuncDec(String,Vec<(String,Typ)>,Typ,Vec<Stmt>),
    VarDec(Typ,String),
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
    If(Box<Exp>,Vec<Stmt>,Option<Vec<Stmt>>),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Dec{
    FuncDec(String,Vec<(Typ,String)>,Typ,Stmt),
    VarDec(Typ,String)
}

#[derive(Debug,Clone,PartialEq)]
pub enum Typ {
    NameTyp(String),
    ArrayTyp(i64,Box<Typ>),
    StrTyp,
    IntTyp,
    VoidTyp,
}


