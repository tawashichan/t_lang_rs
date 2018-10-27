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
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            if l == "" {
                let tokens = lexer::str_to_tokens(&s);
                let ast = parser::parse(&tokens);
                println!("{:?}", ast);
                let obj = eval::eval_prog(ast);
                println!("{:?}",obj);
                s = format!("");
            } else {
                s = s + &l;
            }
        }
    }
}