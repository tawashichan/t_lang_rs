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
        &[] => (&[],stmts.to_vec()),
        _ => {
            let (res,stmt) = parse_stmt(tokens);
            stmts.push(stmt);
            parse_stmts(res,stmts)
        }
    }
}

fn parse_stmt(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::LET,Token::VAR(s),Token::EQUAL,rest..] => {
            let (res,exp) = parse_exp(rest);
            (res,Stmt::Assign(Var::Var(s.clone()),exp))
        },
        [Token::FUNCTION,rest..] => parse_function(tokens)
        ,
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(tokens);
            (res,Stmt::CallProc(s.clone(),args))
        },
        [Token::RETURN,rest..] => {
            let (res,exp) = parse_exp(rest);
            (res,Stmt::CallProc(format!("return"),vec![exp]))
        },
        _ => {
            let (rest,exp) = parse_exp(tokens);
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
        [Token::VAR(s),rest..] => {
            let (res,typ) = get_type(rest);
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
                },
                _ => panic!("{:?}",res)
            }
        },
        _ => panic!("{:?}",tokens)
    }
}

fn parse_func_call_arg<'a>(tokens: &'a [Token], args: &mut Vec<Exp>) -> (&'a [Token], Vec<Exp>) {
    let (rest, exp) = parse_exp(tokens);
    args.push(exp);
    match rest {
        [Token::COMMA, res..] => parse_func_call_arg(res, args),
        [Token::RPAR,res..] => (rest,args.to_vec()),
        _ => panic!("{:?}",rest)//(rest, args.to_vec())
    }
}

fn parse_type_str(s: &str) -> Typ {
    match s {
        "Int" => Typ::IntTyp,
        _ => Typ::IntTyp
    }
}

fn get_type(tokens: &[Token]) -> (&[Token],Typ){
    match tokens {
        [Token::VAR(s),rest..] => (rest,parse_type_str(s)),
        _ => panic!()
    }
}

/*pub fn parse_exp(tokens: &[Token],exp: Option<Exp>) -> (&[Token],Exp) {
    match exp {
        Some(exp) => {
            match tokens {
                [Token::PLUS,rest..] => {
                    let (res,ex) = parse_exp(rest,None);
                    (res,Exp::CallFunc(format!("+"),vec![exp,ex]))
                },
                [Token::MINUS,rest..] => {
                    let (res,ex) = parse_exp(rest,None);
                    (res,Exp::CallFunc(format!("-"),vec![exp,ex]))
                },
                [Token::MUL,rest..] => {
                    let (res,ex) = parse_exp(rest,None);
                    (res,Exp::CallFunc(format!("*"),vec![exp,ex]))
                },
                [Token::DIV,rest..] => {
                    let (res,ex) = parse_exp(rest,None);
                    (res,Exp::CallFunc(format!("/"),vec![exp,ex]))
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
                [Token::VAR(s),rest..] =>
                    parse_exp(rest,Some(Exp::VarExp(box Var::Var(s.clone())))),
                _ => {
                    panic!("{:?}",tokens)
                }
            }
        }
    }
}*/

fn parse_exp(tokens: &[Token]) -> (&[Token],Exp) {
    match tokens {
        [Token::INT(i),rest..] => {
            parse_op_expr(rest,Exp::IntExp(*i))
        },
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(tokens);
            parse_op_expr(res,Exp::CallFunc(s.clone(),args))
        },
        [Token::VAR(s),rest..] =>
            parse_op_expr(rest,Exp::VarExp(box Var::Var(s.clone()))),
        [Token::LPAR,rest..] => {
            let (res,exp) = parse_exp(rest);
            match res {
                [Token::RPAR,re..] => {
                    parse_op_expr(re,exp)
                },
                _ => {
                    panic!("{:?}",res)
                }
            }
        }
        _ => {
            parse_exp(&[])
        }
    }
}

fn parse_mul_div_expr(tokens: &[Token]) -> (&[Token], Exp) {
    match tokens {
        [Token::INT(i), rest..] => {
            (rest, Exp::IntExp(*i))
        }
        [Token::VAR(s), Token::LPAR, rest..] => {
            let (res, args) = parse_func_call_args(tokens);
            (res, Exp::CallFunc(s.clone(), args))
        }
        [Token::VAR(s), rest..] =>
            (rest, Exp::VarExp(box Var::Var(s.clone()))),
        [Token::LPAR, rest..] => {
            let (res, exp) = parse_exp(rest);
            match res {
                [Token::RPAR, re..] => {
                    (res, exp)
                }
                _ => {
                    panic!("{:?}", res)
                }
            }
        }
        _ => {
            panic!("{:?}",tokens)
        }
    }
}

fn parse_op_expr(tokens: &[Token],exp: Exp) -> (&[Token],Exp) {
    match tokens {
        [Token::PLUS,rest..] => {
            let (res,ex) = parse_exp(rest);
            (res,Exp::CallFunc(format!("+"),vec![exp,ex]))
        },
        [Token::MINUS,rest..] => {
            let (res,ex) = parse_exp(rest);
            (res,Exp::CallFunc(format!("-"),vec![exp,ex]))
        },
        [Token::MUL,rest..] => {
            let (res,ex) = parse_mul_div_expr(rest);
            parse_op_expr(res,Exp::CallFunc(format!("*"),vec![exp,ex]))
        },
        [Token::DIV,rest..] => {
            let (res,ex) = parse_mul_div_expr(rest);
            parse_op_expr(res,Exp::CallFunc(format!("/"),vec![exp,ex]))
        },
        _ => {
            (tokens,exp)
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


