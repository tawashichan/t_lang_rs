use crate::ast::{Exp, Prog, Stmt, Typ};

pub fn code_gen(ast: Prog) -> String {
    gen_main(ast)
}

fn gen_main(prog: Prog) -> String {
    format!(
        ".intel_syntax noprefix
.global plus
plus:
    {}
    add rsi, rdi
    mov rax, rsi
    {}
.global minus
minus:
    {}
    sub rsi, rdi
    mov rax, rsi
    {}
{}
",
        gen_func_prologue(8),
        gen_func_epilogue(),
        gen_func_prologue(8),
        gen_func_epilogue(),
        gen_prog(prog)
    )
}

fn gen_prog(prog: Prog) -> String {
    prog.stmts.into_iter().fold("".to_string(), |acm, current| {
        format!("{}{}", acm, gen_stmt(&current))
    })
}

fn gen_stmt(stmt: &Stmt) -> String {
    match stmt {
        Stmt::ExpStmt(exp) => gen_exp(&exp),
        Stmt::FuncDec(name, args, ret, body) => gen_func_dec(&name, &args, &ret, &body),
        Stmt::Return(exp) => {
            let e = gen_exp(&exp);
            format!("mov rax, {}",e)
        },
        Stmt::Block(stmts) => stmts.into_iter().fold("".to_string(), |acm, current| {
            format!("{}{}", acm, gen_stmt(&current))
        }),
        _ => format!(""),
    }
}

fn gen_exp(exp: &Exp) -> String {
    match exp {
        Exp::IntExp(i) => format!("{}", i),
        Exp::CallFunc(name, args) => gen_call_func(&name, &args),
        _ => "".to_string(),
    }
}

fn gen_func_dec(name: &str, args: &[(String, Typ)], ret: &Typ, body: &Stmt) -> String {
    let name = if name == "main" { "_main" } else { name };
    let args_size = (args.len() * 4) as i64; // とりあえず全ての変数のサイズは4byteであるとしておく。
    format!(
        "
.text
.global {}
{}:
    {}
    {}
    {}
    ",
        name,
        name,
        gen_func_prologue(args_size),
        gen_stmt(body),
        gen_func_epilogue()
    )
}

fn gen_call_func(name: &str, args: &[Exp]) -> String {
    match name {
        "+" => {
            if args.len() != 2 {
                panic!("")
            };
            let e1 = gen_exp(&args[0]);
            let e2 = gen_exp(&args[1]);
            format!(
                "
            mov rdi, {}
            mov rsi, {}
            call plus
            ",
                e1, e2
            )
        }
        _ => {
            let args_code = args
                .iter()
                .fold("".to_string(), |acm, current| acm + "\nmov rdi, 1");
            format!(
                "
            {}
            call {}
            ",
                args_code, name
            )
        }
    }
}

fn gen_func_prologue(diff: i64) -> String {
    format!(
        "push rbp
    mov rbp, rsp
    sub rsp, {}",
        diff
    )
}

fn gen_func_epilogue() -> String {
    format!(
        "mov rsp, rbp
    pop rbp
    ret"
    )
}
