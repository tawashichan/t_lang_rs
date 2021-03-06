use std::collections::HashMap;
use crate::ast::{Prog,Exp,Stmt,Var,Typ,init_prog};
use crate::lexer::{Token};

#[derive(Debug)]
enum CheckNext {
    Stmt,
    Expr,
    End,
}

//現在読み取り中の位置を保持するmutableな構造体使った方が確実に実装は楽

pub fn parse(tokens: &[Token]) -> Prog {
    let (rest,stmts) = parse_stmts(tokens,&mut vec![]);     
    if rest.len() > 0 { panic!("あまったよ {:?}",rest) }
    init_prog(stmts)
}

fn parse_block(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::LBRACE,rest..] => {
            let (res,stmts) = parse_stmts(rest,&mut vec![]);
            match res {
                [Token::RBRACE,re..] => (re,Stmt::Block(stmts)),
                _ => panic!("{:?}",res)
            }
        }
        _ => panic!()
    }
}

fn parse_stmts<'a>(tokens: &'a[Token],stmts: &mut Vec<Stmt>) -> (&'a[Token],Vec<Stmt>) {
    let next = check_next(tokens);
    match next {
        CheckNext::Stmt => {
            let (rest,stmt) = parse_stmt(tokens);
            stmts.push(stmt);
            parse_stmts(rest, stmts)
        },
        CheckNext::Expr => {
            let (rest,exp) = parse_exp(tokens);
            let exp_stmt = Stmt::ExpStmt(exp);
            stmts.push(exp_stmt);
            parse_stmts(rest, stmts)
        },
        CheckNext::End => {
            (tokens,stmts.to_vec())
        }
    }
}

fn parse_stmt(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::LET,Token::VAR(s),Token::EQUAL,rest..] => {
            let (res,exp) = parse_exp(rest);
            (res,Stmt::Assign(Var::Var(s.clone()),exp))
        }
        [Token::FUNCTION,rest..] => parse_function(tokens),
        [Token::RETURN,rest..] => {
            let (res,exp) = parse_exp(rest);
            (res,Stmt::Return(exp))
        }
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(tokens);
            (res,Stmt::CallProc(s.clone(),args))
        }
        [Token::STRUCT,rest..] => parse_struct(tokens),
        [Token::LBRACE,rest..] => {
            let (res,block) = parse_block(tokens);
            (res,block)
        },
        _ => {
            let (rest,exp) = parse_exp(tokens);
            (rest,Stmt::ExpStmt(exp))
        }
    } 
}

// 先読みを共通化したほうが良いかもしれない
fn check_next(tokens: &[Token]) -> CheckNext {
    match tokens {
        [Token::LET,rest..] => CheckNext::Stmt,
        [Token::FUNCTION,rest..] => CheckNext::Stmt,
        //[Token::VAR(s),rest..] => CheckNext::Stmt,
        [Token::RETURN,rest..] => CheckNext::Stmt,
        [Token::STRUCT,rest..] => CheckNext::Stmt,
        [Token::LBRACE,rest..] => CheckNext::Stmt,
        [Token::LPAR,rest..] => CheckNext::Expr,
        [Token::IF,rest..] => CheckNext::Expr,
        [Token::INT(i),rest..] => CheckNext::Expr,
        [Token::VAR(s),Token::LPAR,rest..] => CheckNext::Expr,
        [Token::VAR(s),rest..] => CheckNext::Expr,
        [Token::STRING(s), rest..] => CheckNext::Expr,
        [Token::NOT,rest..] => CheckNext::Expr,
        [Token::TRUE,rest..] => CheckNext::Expr,
        [Token::FALSE,rest..] => CheckNext::Expr,
        [Token::LBRACKET,rest..] => CheckNext::Expr,
        _ => CheckNext::End,
    }
}



fn parse_struct(tokens: &[Token]) -> (&[Token],Stmt){
    match tokens {
        [Token::STRUCT,Token::VAR(s),Token::LBRACE,rest..] => {
            let mut map = HashMap::new();
            let (res,contents) = parse_struct_contents(rest,&mut map);
            match res {
                [Token::RBRACE,re..] => (re,Stmt::StructDec(s.clone(),contents.to_owned())),
                _ => panic!("{:?}",res)
            }
        },
        _ => panic!("{:?}",tokens)
    }
}

// tokensとcontentsのライフタイムは異なる
fn parse_struct_contents<'a,'b>(tokens: &'a[Token],contents: &'b mut HashMap<String,Typ>) -> (&'a[Token],&'b mut HashMap<String,Typ>){
    match tokens {
        [Token::VAR(s),Token::COLON,rest..] => {
            let (res,typ) = parse_type(rest);
            contents.insert(s.clone(),typ);
            parse_struct_contents(res,contents)
        }
        [Token::COMMA,rest..] => {
            parse_struct_contents(rest,contents)
        }
        _ => (tokens,contents)
    }
}

fn parse_function(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::FUNCTION,Token::VAR(s),rest..] => {
            let (res,args) = parse_func_def_args(rest);
            let (re,typ) = parse_type(res);
            let (r,block) = parse_block(re);
            (r,Stmt::FuncDec(s.clone(),args,typ,box block))
        }
        _ => panic!()
    }
}

fn parse_func_def_args(tokens: &[Token]) -> (&[Token],Vec<(String,Typ)>) {
    match tokens {
        [Token::LPAR,rest..] => {
            let (res,args) = parse_func_def_arg(rest,&mut vec![]);
            match res {
                [Token::RPAR,re..] => (re,args),
                _ => panic!() // )で閉じられていなかったらpanic
            }
        }
        _ => panic!()
    }
}

fn parse_func_def_arg<'a>(tokens: &'a[Token],args: &mut Vec<(String,Typ)>) -> (&'a[Token],Vec<(String,Typ)>) {
    match tokens {
        [Token::COMMA,rest..] => parse_func_def_arg(rest,args),
        [Token::RPAR,rest..] => (tokens,args.to_vec()),
        [Token::VAR(s),rest..] => {
            let (res,typ) = parse_type(rest);
            args.push((s.clone(),typ));
            parse_func_def_arg(res,args)
        }
        _ => panic!("{:?}",tokens) //(tokens,args.to_vec())
    }
}

fn parse_func_call_args(tokens: &[Token]) -> (&[Token],Vec<Exp>) {
    match tokens {
        [Token::VAR(s),Token::LPAR,rest..] =>  {
            let (res,args) = parse_func_call_arg(rest,&mut vec![]);
            match res {
                [Token::RPAR,re..] => {
                    (re,args)
                }
                _ => panic!("{:?}",res)
            }
        }
        _ => panic!("{:?}",tokens)
    }
}

fn parse_func_call_arg<'a>(tokens: &'a [Token], args: &mut Vec<Exp>) -> (&'a [Token], Vec<Exp>) {
    match tokens {
        [Token::RPAR,rest..] => (tokens, args.to_vec()),
        [Token::COMMA, rest..] => parse_func_call_arg(rest, args),
        _ => {
            let (rest, exp) = parse_exp(tokens);
            args.push(exp);
            parse_func_call_arg(rest, args)
        }
    }
}

fn parse_type_str(s: &str) -> Typ {
    match s {
        "Int" => Typ::IntTyp,
        "String" => Typ::StrTyp,
        "Bool" => Typ::BoolTyp,
        s => Typ::NameTyp(s.to_string())
    }
}

fn parse_type(tokens: &[Token]) -> (&[Token],Typ){
    match tokens {
        [Token::VAR(s),rest..] => (rest,parse_type_str(s)),
        [Token::LBRACE,rest..] => (tokens,Typ::VoidTyp),
        _ => panic!()
    }
}

fn parse_exp(tokens: &[Token]) -> (&[Token],Exp) {
    let (rest,exp) = parse_op_exp(tokens);
    match rest {
        [Token::PLUS,res..] => {
            let (re,ex) = parse_exp(res);
            (re,Exp::CallFunc(format!("+"),vec![exp,ex]))
        },
        [Token::MINUS,res..] => {
            let (re,ex) = parse_exp(res);
            (re,Exp::CallFunc(format!("-"),vec![exp,ex]))
        },
        [Token::EQUAL,rest..] => {
            let (res,ex) = parse_exp(rest);
            (res,Exp::CallFunc(format!("="),vec![exp,ex]))
        },
        [Token::GT,rest..] => {
            let (res,ex) = parse_exp(rest);
            (res,Exp::CallFunc(format!(">"),vec![exp,ex]))
        },
        [Token::LT,rest..] => {
            let (res,ex) = parse_exp(rest);
            (res,Exp::CallFunc(format!("<"),vec![exp,ex]))
        },
        _ => (rest,exp)
    }
}

fn parse_op_exp(tokens: &[Token]) -> (&[Token],Exp) {
    let (rest,exp) = parse_term(tokens);
    match rest {
        [Token::MUL,res..] => {
            let (re,ex) = parse_op_exp(res);
            (re,Exp::CallFunc(format!("*"),vec![exp,ex]))
        },
        [Token::DIV,res..] => {
            let (re,ex) = parse_op_exp(res);
            (re,Exp::CallFunc(format!("/"),vec![exp,ex]))
        },
        _ => (rest,exp)
    }
} 

fn parse_term(tokens: &[Token]) -> (&[Token],Exp) {
    match tokens {
        [Token::LPAR,rest..] => {
            let (res,exp) = parse_exp(rest);
            match res {
                [Token::RPAR,re..] => {
                    (re,exp)
                },
                _ => {
                    panic!("{:?}",res)
                }
            }
        }
        [Token::IF,rest..] => {
            let (res,cond) = parse_exp(rest);
            let (re,then) = parse_block(res);
            match re {
                [Token::ELSE,r..] => {
                    let (rr,els) = parse_block(r);
                    (rr,Exp::If(box cond,box then,box Some(els)))
                }
                _ => {
                    (re,Exp::If(box cond,box then,box None))
                }
            }
        }
        [Token::INT(i),rest..] => {
            (rest,Exp::IntExp(*i))
        }
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(tokens);
            (res,Exp::CallFunc(s.clone(),args))
        }
        [Token::VAR(s),rest..] => {
            (rest,Exp::VarExp(box Var::Var(s.clone())))
        }
        [Token::STRING(s), rest..] =>
            (rest, Exp::StrExp(s.clone())),
        [Token::NOT,rest..] => {
            let (res,exp) = parse_exp(rest);
            (res,Exp::CallFunc(format!("!"),vec![exp]))
        }
        [Token::TRUE,rest..] => {
            (rest,Exp::BoolExp(true))
        }
        [Token::FALSE,rest..] => {
            (rest,Exp::BoolExp(false))
        }
        [Token::LBRACKET,rest..] => {
            let (rest,array) = parse_array(rest,&mut vec![]);
            (rest,Exp::ArrayExp(array))
        }
        _ => {
            panic!("{:?}",tokens)
        }
    }
}

fn parse_array<'a>(tokens: &'a[Token],acm: &mut Vec<Exp>) -> (&'a[Token],Vec<Exp>) {
    match tokens {
        [Token::RBRACKET,rest..] => (rest,acm.clone()),
        [Token::COMMA,rest..] => parse_array(rest, acm),
        _ => {
            let (rest,exp) = parse_exp(tokens);
            acm.push(exp);
            parse_array(rest, acm)
        }
    }
}


#[test]
fn parse_exp1(){
    let tokens = vec![Token::LPAR,Token::INT(10),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::IntExp(10))
}

#[test]
fn parse_exp2(){
    let tokens = vec![Token::INT(10)];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::IntExp(10))
}

#[test]
fn parse_exp3(){
    let tokens = vec![Token::INT(10),Token::PLUS,Token::INT(11)];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(10),Exp::IntExp(11)]))
}

#[test]
fn parse_exp4(){
    let tokens = vec![Token::INT(10),Token::PLUS,Token::LPAR,Token::INT(11),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(10),Exp::IntExp(11)]))
}

#[test]
fn parse_exp5(){
    let tokens = vec![Token::LPAR,Token::LPAR,Token::INT(7),Token::RPAR,Token::PLUS,Token::INT(4),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(7),Exp::IntExp(4)]))
}

#[test]
fn parse_exp7() {
    let result = std::panic::catch_unwind(|| {
        let tokens = vec![Token::LPAR,Token::LPAR, Token::INT(10), Token::RPAR];
        let (rest, exp) = parse_exp(&tokens);
        }
    );
    assert!(result.is_err());
}

#[test]
fn parse_exp8(){
    let tokens = vec![Token::LPAR,Token::INT(7),Token::PLUS,Token::INT(4),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(7),Exp::IntExp(4)]))
}

#[test]
fn parse_exp9(){
    let tokens = vec![Token::INT(1),Token::PLUS,Token::INT(2),Token::MUL,Token::INT(3)];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(1),Exp::CallFunc(format!("*"),vec![Exp::IntExp(2),Exp::IntExp(3)])]))
}

#[test]
fn parse_exp10(){
    let tokens = vec![Token::LPAR,Token::INT(1),Token::PLUS,Token::INT(2),Token::RPAR,Token::MUL,Token::INT(3)];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("*"),vec![Exp::CallFunc(format!("+"),vec![Exp::IntExp(1),Exp::IntExp(2)]),Exp::IntExp(3)]))
}

#[test]
fn parse_exp11(){
    let tokens = vec![Token::INT(1),Token::DIV,Token::INT(1),Token::MINUS,Token::INT(1)];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("-"),vec![Exp::CallFunc(format!("/"),vec![Exp::IntExp(1),Exp::IntExp(1)]),Exp::IntExp(1)]))
}

#[test]
fn parse_exp12(){
    let tokens = vec![Token::LPAR,Token::INT(1),Token::DIV,Token::INT(1),Token::MINUS,Token::INT(1),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("-"),vec![Exp::CallFunc(format!("/"),vec![Exp::IntExp(1),Exp::IntExp(1)]),Exp::IntExp(1)]))
}

#[test]
fn parse_exp13(){
    let tokens = vec![Token::LPAR,Token::LPAR,Token::INT(1),Token::RPAR,Token::DIV,Token::INT(1),Token::MINUS,Token::INT(1),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("-"),vec![Exp::CallFunc(format!("/"),vec![Exp::IntExp(1),Exp::IntExp(1)]),Exp::IntExp(1)]))
}


#[test]
fn parse_exp14(){
    let tokens = vec![Token::TRUE,Token::EQUAL,Token::FALSE];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("="),vec![Exp::BoolExp(true),Exp::BoolExp(false)]))
}

#[test]
fn parse_exp15(){
    let tokens = vec![Token::IF,Token::TRUE,Token::LBRACE,Token::TRUE,Token::RBRACE,Token::ELSE,Token::LBRACE,Token::TRUE,Token::RBRACE];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::If(box Exp::BoolExp(true),box Stmt::Block(vec![Stmt::ExpStmt(Exp::BoolExp(true))]),box Some(Stmt::Block(vec![Stmt::ExpStmt(Exp::BoolExp(true))]))))
}

#[test]
fn parse_exp16(){
    let tokens = vec![Token::IF,Token::TRUE,Token::LBRACE,Token::TRUE,Token::RBRACE,Token::ELSE,Token::LBRACE,Token::TRUE,Token::RBRACE];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::If(box Exp::BoolExp(true),box Stmt::Block(vec![Stmt::ExpStmt(Exp::BoolExp(true))]),box Some(Stmt::Block(vec![Stmt::ExpStmt(Exp::BoolExp(true))]))))
}

#[test]
fn parse_exp17(){
    let tokens = vec![Token::STRUCT,Token::VAR(format!("Hoge")),Token::LBRACE,Token::VAR(format!("hoge")),Token::COLON,Token::VAR(format!("Int")),Token::RBRACE];
    let (rest,stmt) = parse_stmt(&tokens);
    let mut content = HashMap::new();
    content.insert(format!("hoge"),Typ::IntTyp);
    assert_eq!(stmt,Stmt::StructDec(format!("Hoge"),content))
}

#[test]
fn parse_exp18() {
    let tokens = vec![Token::INT(10), Token::INT(11)];
    let (rest, exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::IntExp(10));
}

#[test]
fn parse_exp19() {
    let tokens = vec![Token::LBRACKET,Token::INT(10), Token::COMMA,Token::INT(11),Token::RBRACKET];
    let (rest, exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::ArrayExp(vec![Exp::IntExp(10),Exp::IntExp(11)]));
}


#[test]
fn parse_exp_ex1(){
    let tokens = vec![Token::LPAR,Token::LPAR,Token::INT(1),Token::RPAR,Token::DIV,Token::INT(1),Token::MINUS,Token::INT(1),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens);
    assert_eq!(exp,Exp::CallFunc(format!("-"),vec![Exp::CallFunc(format!("/"),vec![Exp::IntExp(1),Exp::IntExp(1)]),Exp::IntExp(1)]))
}

#[test]
fn parse_stmts1() {
    let tokens = vec![Token::LET,Token::VAR("hoge".to_string()), Token::EQUAL,Token::INT(5),Token::LET,Token::VAR("hoge".to_string()),Token::EQUAL,Token::INT(8)];
    let (rest, stmts) = parse_stmts(&tokens,&mut vec![]);
    assert_eq!(stmts,vec![Stmt::Assign(Var::Var("hoge".to_string()),Exp::IntExp(5)),Stmt::Assign(Var::Var("hoge".to_string()),Exp::IntExp(8))]);
}

#[test]
fn parse_stmts2() {
    let tokens = vec![Token::LET,Token::VAR("hoge".to_string()), Token::EQUAL,Token::INT(5),Token::LBRACE,Token::LET,Token::VAR("hoge".to_string()),Token::EQUAL,Token::INT(8),Token::RBRACE];
    let (rest, stmts) = parse_stmts(&tokens,&mut vec![]);
    assert_eq!(stmts,vec![Stmt::Assign(Var::Var("hoge".to_string()),Exp::IntExp(5)),Stmt::Block(vec![Stmt::Assign(Var::Var("hoge".to_string()),Exp::IntExp(8))])]);
}

#[test]
fn parse_stmts3() {
    let tokens = vec![Token::LET,Token::VAR("hoge".to_string()), Token::EQUAL,Token::VAR("huga".to_string())];
    let (rest, stmts) = parse_stmts(&tokens,&mut vec![]);
    assert_eq!(stmts,vec![Stmt::Assign(Var::Var("hoge".to_string()),Exp::VarExp(box Var::Var("huga".to_string())))]);
}


#[test]
fn parse_stmts4() {
    let tokens = vec![Token::LBRACE,Token::VAR("hoge".to_string()),Token::RBRACE];
    let (rest, stmts) = parse_stmts(&tokens,&mut vec![]);
    assert_eq!(stmts,vec![Stmt::Block(vec![Stmt::ExpStmt(Exp::VarExp(box Var::Var("hoge".to_string())))])]);
}