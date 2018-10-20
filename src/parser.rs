use ast::{Prog,Exp,Stmt,Var,Dec,Typ};
use lexer::{Token};


//現在読み取り中の位置を保持するmutableな構造体使った方が確実に実装は楽

pub fn parse(tokens: &[Token]) -> Prog {
    let(rest,stmts) = parse_stmts(tokens,&mut vec![]);
    Prog{ stmts }
}

fn parse_stmts<'a>(tokens: &'a[Token],stmts: &mut Vec<Stmt>) -> (&'a[Token],Vec<Stmt>) {
    match tokens {
        [first,rest..] => {
            let (res,stmt) = parse_stmt(tokens);
            stmts.push(stmt);
            parse_stmts(res,stmts)
        }
        &[] => (&[],stmts.to_vec()),
        _ => (&[],stmts.to_vec())
    }
}

fn parse_stmt(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::LET,Token::STRING(s),Token::EQUAL,rest..] => {
            let (res,exp) = parse_expr(rest,None);
            (res,Stmt::Assign(Var::Var(s.clone()),exp))
        },
        [Token::FUNCTION,Token::STRING(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_def_args(rest);
            let (re,typ,stmts) = parse_func_type_body(res);
            (re,Stmt::FuncDec(s.clone(),args,typ,stmts))
        },
        [first,rest..] => panic!("invalid token {:?}",first),
        &[] => panic!()
    }
}

fn parse_func_def_args(tokens: &[Token]) -> (&[Token],Vec<(String,Typ)>) {
    parse_func_def_arg(tokens,&mut vec![])
}

fn parse_func_def_arg<'a>(tokens: &'a[Token],args: &mut Vec<(String,Typ)>) -> (&'a[Token],Vec<(String,Typ)>) {
    println!("{:?}",tokens);
    match tokens {
        [Token::RPAR,rest..] => (rest,args.to_vec()),
        [Token::STRING(s),Token::STRING(ts),rest..] => {
            args.push((s.clone(),parse_type_str(ts.clone())));
            parse_func_def_arg(rest,args)
        }
        _ => (&[],args.to_vec())
    }
}

fn parse_type_str(s: String) -> Typ {
    match s.as_str() {
        "Int" => Typ::IntTyp,
        _ => Typ::IntTyp
    }
}

fn parse_func_type_body(tokens: &[Token]) -> (&[Token],Typ,Vec<Stmt>){
    match tokens {
        [Token::STRING(t),Token::LBRACE,rest..] => {
            let typ = parse_type_str(t.clone());
            let (res,stmts) = parse_stmts(rest,&mut vec![]);
            (res,typ,stmts.to_vec())
        }
        _ => panic!("func body")
    }
}

fn parse_expr(tokens: &[Token],exp: Option<Exp>) -> (&[Token],Exp) {
    println!("{:?}",tokens);
    match tokens {
        [Token::INT(i),rest..] => parse_expr(rest,Some(Exp::IntExp(*i))),
        [Token::PLUS,rest..] => {
            let (res,e) = parse_expr(rest,None);
            (res,Exp::CallFunc(format!("+"),vec![exp.unwrap(),e]))},
        [Token::MINUS,rest..] => {
            let (res,e) = parse_expr(rest,None);
            (res,Exp::CallFunc(format!("-"),vec![exp.unwrap(),e]))},
        _ => (tokens,exp.unwrap())
    }
}

