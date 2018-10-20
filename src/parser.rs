use ast::{Prog,Exp,Stmt,Var,Dec,Typ};
use lexer::{Token};


//現在読み取り中の位置を保持するmutableな構造体使った方が確実に実装は楽

pub fn parse(tokens: &[Token]) -> Prog {
    let(rest,stmts) = parse_stmts(tokens,&mut vec![]);
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
        _ => (&[],stmts.to_vec())
    }
}

fn parse_stmt(tokens: &[Token]) -> (&[Token],Stmt) {
    match tokens {
        [Token::LET,Token::VAR(s),Token::EQUAL,rest..] => {
            let (res,exp) = parse_expr(rest);
            (res,Stmt::Assign(Var::Var(s.clone()),exp))
        },
        [Token::FUNCTION,Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_def_args(rest);
            let (re,typ) = get_type(res);
            let (r,stmts) = parse_stmts(re,&mut vec![]);
            (r,Stmt::FuncDec(s.clone(),args,typ,stmts))
        },
        [Token::VAR(s),Token::LPAR,rest..] => {
            let (res,args) = parse_func_call_args(rest);
            (res,Stmt::CallProc(s.clone(),args))
        },
        [Token::RETURN,rest..] => {
            let (res,exp) = parse_expr(rest);
            (res,Stmt::CallProc(format!("return"),vec![exp]))
        },
        [first,rest..] => panic!("invalid token {:?}",first),
        &[] => panic!()
    }
}

fn parse_func_def_args(tokens: &[Token]) -> (&[Token],Vec<(String,Typ)>) {
    parse_func_def_arg(tokens,&mut vec![])
}

fn parse_func_def_arg<'a>(tokens: &'a[Token],args: &mut Vec<(String,Typ)>) -> (&'a[Token],Vec<(String,Typ)>) {
    match tokens {
        [Token::RPAR,rest..] => (rest,args.to_vec()),
        [Token::VAR(s),Token::VAR(ts),rest..] => {
            args.push((s.clone(),parse_type_str(ts.clone())));
            parse_func_def_arg(rest,args)
        }
        _ => (&[],args.to_vec())
    }
}

fn parse_func_call_args(tokens: &[Token]) -> (&[Token],Vec<Exp>) {
    parse_func_call_arg(tokens,&mut vec![])
}

fn parse_func_call_arg<'a>(tokens: &'a [Token], args: &mut Vec<Exp>) -> (&'a [Token], Vec<Exp>) {
    let (rest, exp) = parse_expr(tokens);
    args.push(exp);
    match rest {
        [Token::COMMA, res..] => parse_func_call_arg(res, args),
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

fn parse_expr_sub(tokens: &[Token]) -> (&[Token],Exp) {
    match tokens {
        [Token::LPAR,rest..] => {
            let (res,exp) = parse_expr_sub(rest);
            println!("{:?}",res);
            match res {
                [Token::RPAR,re..] => (re,exp),
                _ => panic!()
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
