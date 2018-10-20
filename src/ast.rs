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

#[derive(Debug)]
pub struct Prog{
    pub stmts: Vec<Stmt>
}

#[derive(Debug,Clone)]
pub enum Var {
    Var(String),
    IndexedVar(Box<Var>,Exp)
}
//一旦宣言を
#[derive(Debug,Clone)]
pub enum Stmt {
    Assign(Var,Exp),
    CallProc(String,Vec<Exp>),
    If(Exp,Box<Stmt>,Option<Box<Stmt>>),
    //Block(Vec<Dec>,Vec<Stmt>),
    FuncDec(String,Vec<(String,Typ)>,Typ,Vec<Stmt>),
    VarDec(Typ,String),
    NilStmt
}

#[derive(Debug,Clone)]
pub enum Exp {
    VarExp(Box<Var>),
    StrExp(String),
    IntExp(i64),
    CallFunc(String,Vec<Exp>)
}

#[derive(Debug,Clone)]
pub enum Dec{
    FuncDec(String,Vec<(Typ,String)>,Typ,Stmt),
    VarDec(Typ,String)
}

#[derive(Debug,Clone)]
pub enum Typ {
    NameTyp(String),
    ArrayTyp(i64,Box<Typ>),
    IntTyp,
    VoidTyp,
}


