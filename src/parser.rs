use ast::{Prog,Exp,Stmt,Var,Dec,Typ};
use lexer::{Token};


//現在読み取り中の位置を保持するmutableな構造体使った方が確実に実装は楽

pub fn parse(tokens: &[Token]) -> Prog {
    let(rest,stmts) = parse_stmts(tokens,&mut vec![]);
    if rest.len() > 0 { panic!("あまったよ {:?}",rest) }
    Prog{ stmts }
}

fn parse_stmts<'a>(tokens: &'a[Token],stmts: &mut Vec<Stmt>) -> (&'a[Token],Vec<Stmt>) {
    match tokens {
        [Token::LBRACE,rest..] => parse_stmts(rest,&mut vec![]), // ここあまりきれいじゃない...
        [Token::RBRACE,rest..] => (rest,stmts.to_vec()),
        [first,rest..] => {
            let (res,stmt) = parse_stmt(tokens);
            stmts.push(stmt);
            parse_stmts(res,stmts)
        }
        &[] => (&[],stmts.to_vec()),
        _ => (tokens,stmts.to_vec())
    }
}

fn parse_stmt(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::LET,Token::VAR(s),Token::EQUAL,rest..] => {
            let (res,exp) = parse_exp(rest,None);
            (res,Stmt::Assign(Var::Var(s.clone()),exp))
        },
        [Token::FUNCTION,rest..] => parse_function(tokens)
        ,
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(tokens);
            (res,Stmt::CallProc(s.clone(),args))
        },
        [Token::RETURN,rest..] => {
            let (res,exp) = parse_exp(rest,None);
            (res,Stmt::CallProc(format!("return"),vec![exp]))
        },
        _ => {
            let (rest,exp) = parse_exp(tokens,None);
            (rest,Stmt::ExpStmt(exp))
        }
    }
}

fn parse_function(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::FUNCTION,Token::VAR(s),rest..] => {
            let (res,args) = parse_func_def_args(rest);
            let (re,typ) = get_type(res);
            let (r,stmts) = parse_stmts(re,&mut vec![]);
            (r,Stmt::FuncDec(s.clone(),args,typ,stmts))
        },
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
        [Token::VAR(s),Token::VAR(ts),rest..] => {
            args.push((s.clone(),parse_type_str(ts.clone())));
            parse_func_def_arg(rest,args)
        }
        _ => (tokens,args.to_vec())
    }
}

fn parse_func_call_args(tokens: &[Token]) -> (&[Token],Vec<Exp>) {
    match tokens {
        [Token::VAR(s),Token::LPAR,rest..] =>  {
            let (res,args) = parse_func_call_arg(rest,&mut vec![]);
            match res {
                [Token::RPAR,re..] => {
                    (re,args)
                },
                _ => panic!("{:?}",res)
            }
        },
        _ => panic!("{:?}",tokens)
    }

}

fn parse_func_call_arg<'a>(tokens: &'a [Token], args: &mut Vec<Exp>) -> (&'a [Token], Vec<Exp>) {
    let (rest, exp) = parse_exp(tokens,None);
    args.push(exp);
    match rest {
        [Token::COMMA, res..] => parse_func_call_arg(res, args),
        [Token::RPAR,res..] => (rest,args.to_vec()),
        _ => (rest, args.to_vec())
    }
}

fn parse_type_str(s: String) -> Typ {
    match s.as_str() {
        "Int" => Typ::IntTyp,
        _ => Typ::IntTyp
    }
}

fn get_type(tokens: &[Token]) -> (&[Token],Typ){
    match tokens {
        [Token::VAR(s),rest..] => (rest,parse_type_str(s.clone())),
        _ => panic!()
    }
}

fn parse_expr(tokens: &[Token]) -> (&[Token],Exp) {
    let (rest,exp) = parse_expr_sub(tokens);
    match rest {
        [Token::PLUS,res..] => {
            let (re,ex) = parse_expr(res);
            (re,Exp::CallFunc(format!("+"),vec![exp,ex]))
        },
        [Token::MINUS,res..] => {
            let (re,ex) = parse_expr(res);
            (re,Exp::CallFunc(format!("-"),vec![exp,ex]))
        },
        [Token::RPAR,res..] => (res,exp),
        _ => (rest,exp)
    }
}

// ()の対応つらすぎ
fn parse_expr_sub(tokens: &[Token]) -> (&[Token],Exp) {
    match tokens {
        [Token::LPAR,rest..] => {
            let (res,exp) = parse_expr_sub(rest);
            match res {
                [Token::RPAR,re..] => (re,exp),
                [Token::PLUS,re..] => (res,exp),
                _ => panic!()//(res,exp)
            }
        },
        [Token::INT(i),rest..] => {
            (rest,Exp::IntExp(*i))
        },
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(rest);
            (res,(Exp::CallFunc(s.clone(),args)))
        },
        [Token::VAR(s),rest..] => (rest,(Exp::VarExp(box Var::Var(s.clone())))),
        _ => {
            panic!()
        }
    }
}


pub fn parse_exp(tokens: &[Token],exp: Option<Exp>) -> (&[Token],Exp) {
    match exp {
        Some(exp) => {
            match tokens {
                [Token::PLUS,rest..] => {
                    let (res,ex) = parse_exp(rest,None);
                    (res,Exp::CallFunc(format!("+"),vec![exp,ex]))
                },
                _ => (tokens,exp)
            }
        },
        None => {
            match tokens {
                [Token::LPAR,rest..] => {
                    let (res,exp) = parse_exp(rest,None);
                    match res {
                        [Token::RPAR,re..] => {
                            parse_exp(re,Some(exp))
                        },
                        _ => {
                            panic!("{:?}",res)
                        }
                    }
                },
                [Token::INT(i),rest..] => {
                    parse_exp(rest,Some(Exp::IntExp(*i)))
                },
                [Token::VAR(s),Token::LPAR,rest..] => {
                    let (res,args) = parse_func_call_args(tokens);
                    parse_exp(res,Some(Exp::CallFunc(s.clone(),args)))

                },
                [Token::VAR(s),rest..] => parse_exp(rest,Some(Exp::VarExp(box Var::Var(s.clone())))),
                _ => {
                    panic!("{:?}",tokens)
                }
            }
        }
    }

}

#[test]
fn parse_exp1(){
    let tokens = vec![Token::LPAR,Token::INT(10),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens,None);
    assert_eq!(exp,Exp::IntExp(10))
}

#[test]
fn parse_exp2(){
    let tokens = vec![Token::INT(10)];
    let (rest,exp) = parse_exp(&tokens,None);
    assert_eq!(exp,Exp::IntExp(10))
}

#[test]
fn parse_exp3(){
    let tokens = vec![Token::INT(10),Token::PLUS,Token::INT(11)];
    let (rest,exp) = parse_exp(&tokens,None);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(10),Exp::IntExp(11)]))
}

#[test]
fn parse_exp4(){
    let tokens = vec![Token::INT(10),Token::PLUS,Token::LPAR,Token::INT(11),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens,None);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(10),Exp::IntExp(11)]))
}

#[test]
fn parse_exp5(){
    let tokens = vec![Token::LPAR,Token::LPAR,Token::INT(7),Token::RPAR,Token::PLUS,Token::INT(4),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens,None);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(7),Exp::IntExp(4)]))
}


#[test]
fn parse_exp7() {
    let result = std::panic::catch_unwind(|| {
        let tokens = vec![Token::LPAR,Token::LPAR, Token::INT(10), Token::RPAR];
        let (rest, exp) = parse_exp(&tokens, None);
        }
    );
    assert!(result.is_err());
}

#[test]
fn parse_exp8(){
    let tokens = vec![Token::LPAR,Token::INT(7),Token::PLUS,Token::INT(4),Token::RPAR];
    let (rest,exp) = parse_exp(&tokens,None);
    assert_eq!(exp,Exp::CallFunc(format!("+"),vec![Exp::IntExp(7),Exp::IntExp(4)]))
}

