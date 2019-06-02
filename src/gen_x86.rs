use crate::ast::{Exp, Prog, Stmt};

pub fn code_gen(ast: Prog) -> String {
    gen_main(ast)
}

fn gen_main(prog: Prog) -> String {
    format!(
        ".intel_syntax noprefix
.global _main,plus
plus:
    {}
    add rsi, rdi
    mov rax, rsi
    {}
minus:
    {}
    sub rsi, rdi
    mov rax, rsi
    {}    
_main:
    {}
    ret",
        gen_func_prologue(8),
        gen_func_epilogue(),
        gen_func_prologue(8),
        gen_func_epilogue(),
        gen_prog(prog)
    )
}

fn gen_prog(prog: Prog) -> String {
    prog.stmts
        .into_iter()
        .fold("".to_string(), |acm, current| format!("{}{}",acm,gen_stmt(current)))
}

fn gen_stmt(stmt: Stmt) -> String {
    match stmt {
        Stmt::ExpStmt(exp) => gen_exp(&exp),
        _ => format!("")
    }
}

fn gen_exp(exp: &Exp) -> String {
    match exp {
        Exp::IntExp(i) => format!("{}", i),
        Exp::CallFunc(name,args) => gen_func(&name, &args),
        _ => "".to_string(),
    }
}

fn gen_func(name: &str,args: &[Exp]) -> String {
    match name {
        "+" => {
            if args.len() != 2 {
                panic!("")
            };
            let e1 = gen_exp(&args[0]);
            let e2 = gen_exp(&args[1]);
            format!("
            mov rdi, {}
            mov rsi, {}
            call plus
            ",e1,e2)
        },
        _ => gen_custom_func(name,args)
    }
}

fn gen_custom_func(name: &str,args: &[Exp]) -> String {
    format!("")
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
