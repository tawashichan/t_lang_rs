use std::io::{self, BufRead};

use parser;
use lexer;
use eval;

//interactive tawashi
pub fn start_itl() {
    println!("start itl");
    loop {
        let mut s = String::new();
        let stdin = io::stdin();
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
                println!("{:?}",obj);
                println!("{:?}",env);
                s = format!("");
            } else {
                s = s + &l;
            }
        }
    }
}