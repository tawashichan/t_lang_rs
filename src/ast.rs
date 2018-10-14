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
/// Node::Statement(Statement::Def(Def::VarDef{name: "hoge".to_string(),var_type: Type::Int,value: Val::Int(1)}))
/// Node::Statement(Statement::Def(Def::FuncDef{name: "huga".to_string(),args: vec![("foo".to_string(),Type::Int)],content: vec![Node::Expr(Expr::TwoTermOp(box TwoTermOp::Plus(Expr::Val(Val::Var("foo".to_string())),Expr::Val(Val::Int(1)))))],return_type: Type::Int}));
/// Node::Statement(Statement::Def(Def::VarDef{name: "aaa".to_string(),var_type: Type::Int,value: Expr::FuncApply(format!("huga"),vec![Expr::Var(format!("hoge"))])}));
/// となる予定。とりあえずこれをコンパイルするのを目標にする。改装深すぎるのでtraitでいい感じにできればなぁ
///
///

#[derive(Debug)]
pub struct  Program {
    pub nodes: Vec<Node>
}

#[derive(Debug)]
pub enum Node {
    Statement(Statement),
    Expr(Expr)
}

#[derive(Debug)]
pub enum Statement {
    Def(Def)
}

#[derive(Debug)]
pub enum Def{
    FuncDef{name: String,args: Vec<(String,Type)>,content: Vec<Node>,return_type: Type},
    VarDef{name: String,value: Expr} //変数は必ず初期化されなければならない
}

#[derive(Debug)]
pub enum TwoTermOp{
    Plus(Expr,Expr),
    Minus(Expr,Expr),
    Mul(Expr,Expr),
    Div(Expr,Expr),
    Equal(Expr,Expr)
}

#[derive(Debug)]
pub enum Val {
    Int(i64),
    String(String),
    Tuple(Vec<Type>), //この二つはどっかで対応する
    Struct(String,Vec<(String,Val)>)
}

#[derive(Debug)]
pub enum Type {
    Int,
    String,
    Array(Vec<Type>),
    Tuple(Vec<Type>),
    Struct(String,Vec<(String,Type)>),
    Void
}

#[derive(Debug)]
pub enum Expr {
    IF{cond: Box<Expr>,then: Box<Expr>,other: Box<Expr>},
    Var(String),
    Val(Val),
    TwoTermOp(Box<TwoTermOp>),
    FuncApply(String,Vec<(Expr)>)
}