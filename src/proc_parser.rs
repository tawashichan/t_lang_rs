/// 手続き的に書くパーサー

use Token;
use ast::*;

pub struct ParseError<'a>{
    pub msg: &'a str
}

pub struct Parser<'a>{
    pub current_location: u64,
    pub peak_location: u64,
    pub tokens: &'a[Token],
    pub error: Option<ParseError<'a>>
}

pub fn parse(tokens: &[Token]) -> Prog {
    let parser = Parser{current_location: 0,peak_location: 0,tokens,error: None};
    Prog{ stmts: vec![] }
}

