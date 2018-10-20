#![feature(slice_patterns)]
#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(nll)]

mod ast;
mod lexer;
mod parser;
mod eval;

fn main() {
    let tokens = lexer::str_to_tokens(sample_string());
    println!("{:?}",tokens);
    let prog = parser::parse(&tokens);
    println!("{:?}",prog);
}

fn sample_string8<'a>() -> &'a str {
    "let a = aa(1,2,3)"
}

fn sample_string7<'a>() -> &'a str {
    "function huga(foo Int) Int {
        return 1 + 1
     }
     return huga(1)
    "
}

fn sample_string6<'a>() -> &'a str {
    "let a = 1 + 1 + 6"
}

fn sample_string5<'a>() -> &'a str {
    "1 + 1"
}

fn sample_string4<'a>() -> &'a str {
    "let fa = 1 + 1
     let aa = 2 + 2
    "
}

fn sample_string3<'a>() -> &'a str {
    "function huga(foo Int) Int {
            foo + 1
        }
    "
}

fn sample_string2<'a>() -> &'a str {
    "let hoge = 1
     let aaa = huga(hoge)
    "

}

fn sample_string<'a>() -> &'a str {

    "
        let hoge = 1

        function huga(foo Int) Int {
            return foo + 1
        }

        let aaa = huga(hoge,tawawa)"

}

/*fn sample_ast(){
    let n = Node::Statement(Statement::Def(Def::VarDef{name: "hoge".to_string(),value: Expr::Int(1)}));
    let n2 = Node::Statement(Statement::Def(Def::FuncDef{name: "huga".to_string(),args: vec![("foo".to_string(),"Int".to_string())],content: vec![Node::Expr(Expr::TwoTermOp(format!("PLUS"),box Expr::Var("foo".to_string()),box Expr::Int(1)))],return_type: Type::Int}));
    let n3 = Node::Statement(Statement::Def(Def::VarDef{name: "aaa".to_string(),value: Expr::FuncApply(format!("huga"),vec![Expr::Var(format!("hoge"))])}));

    let p = Program{nodes: vec![n,n2,n3]};

    println!("{:?}",p);
}*/
