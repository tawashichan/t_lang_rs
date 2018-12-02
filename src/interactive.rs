use std::io::prelude::*;
use std::fs::File;
use std::env;

use parser;
use lexer;
use eval;

//interactive tawashi
pub fn start_itl() {
    println!("start repl");
    let args: Vec<String> = env::args().collect();
    let len = args.len();
    match len {
        1 => init_repl(),
        2 => exec_file(&args[1]),
        _ => panic!("too many arguments")
    }

}

fn exec_file(filename: &str){
    let mut f = File::open(filename).expect("file not found");
    println!("{:?}",f);
    let mut s = String::new();
    // ファイルの読み込み中に問題がありました
    f.read_to_string(&mut s)
        .expect("something went wrong reading the file");
    execute(&s)
}

fn init_repl(){
    loop {
        let mut s = "".to_string();
        let stdin = std::io::stdin();
        let mut env = eval::init_env();
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            if l == "" {
                let tokens = lexer::str_to_tokens(&s);
                println!("{:?}",tokens);
                let ast = parser::parse(&tokens);
                println!("{:?}", ast);
                let (obj,e) = eval::eval_prog(ast,env);
                env = e;
                println!("result: {:?}",obj);
                println!("env: {:?}",env);
                s = format!("");
            } else {
                s = s + &l;
            }
        }
    }
}

fn execute(s: &str) {
    let mut env = eval::init_env();
    let tokens = lexer::str_to_tokens(&s);
    println!("{:?}",tokens);
    let ast = parser::parse(&tokens);
    println!("{:?}", ast);
    let (obj,e) = eval::eval_prog(ast,env);
    println!("{:?}",obj);
    println!("{:?}",e);
}