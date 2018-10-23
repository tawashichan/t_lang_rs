#[derive(Clone,Debug)]
pub enum Token {
    STRING(String),
    INT(i64),
    FLOAT(f64),
    LPAR,
    RPAR,
    LBRACE, // {
    RBRACE, // }
    LBRACKET, // [
    RBRACKET, // ]
    COLON,
    COMMA,
    NULL,
    EQUAL,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LET,
    STRUCT,
    IF,
    ELSE,
    FUNCTION,
    VAR(String),
    BOOLEAN(bool),
    RETURN,
    NOT,
    TRUE,
    FALSE,
    EOF,
}

pub fn split_string(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn get_str(str_vec: &[char]) -> (String, &[char]) {
    get_str_sub(str_vec, "".to_string())
}

fn get_str_sub(str_vec: &[char],acm: String) -> (String,&[char]) {
    match str_vec {
        [first,rest..] => match first {
            '\"' => (acm,rest),
            _c => get_str_sub(rest,format!("{}{}",acm,first))
        }
        &[] => (acm,&[]),
    }
}

fn get_string(str_vec: &[char]) -> (Token,&[char]) {
    match str_vec {
        ['\"',rest..] => get_string_sub(rest,"".to_string()),
        _ => panic!()
    }
}

fn get_string_sub(str_vec: &[char],acm: String) -> (Token,&[char]) {
    match str_vec {
        ['\"',rest..] => (Token::STRING(acm),rest),
        [first,rest..] => get_string_sub(rest,format!("{}{}", acm, first)),
        _ => panic!()
    }
}

fn get_keyword(str_vec: &[char]) -> (Token, &[char]){
    get_keyword_sub(str_vec,"".to_string())
}

fn get_keyword_sub(str_vec: &[char], acm: String) -> (Token, &[char]) {
    match str_vec {
        [first, rest..] => if first.is_alphabetic() {
            get_keyword_sub(rest, format!("{}{}", acm, first))
        } else {
            match &*acm {
                "struct" => (Token::STRUCT, str_vec),
                "let" => (Token::LET, str_vec),
                "if" => (Token::IF, str_vec),
                "else" => (Token::ELSE, str_vec),
                "fun" => (Token::FUNCTION, str_vec),
                "return" => (Token::RETURN,str_vec),
                "==" => (Token::EQUAL,str_vec),
                s =>  (Token::VAR(s.to_string()),str_vec)
            }
        }
        &[] => panic!("invalid tokens")//(acm,&[]),
    }
}

fn get_num_str(str_vec: &[char]) -> (String, &[char],bool,bool) {
    get_num_str_sub(str_vec,"".to_string(),false,false)
}

//jsonの仕様的にintとfloatの区別は存在しないっぽいので
fn get_num_str_sub(str_vec: &[char], acm: String,is_float: bool,is_minus: bool) -> (String, &[char],bool,bool) {
    match &str_vec[..] {
        [first,rest..] => {
            if first.is_numeric() {
                get_num_str_sub(rest, format!("{}{}",acm,first),is_float,is_minus)
            } else if *first == '-' && is_minus == false {
                get_num_str_sub(rest, format!("{}{}",acm,first),is_float,is_minus)
            } else if *first == '.' && is_float == false {
                get_num_str_sub(rest,format!("{}{}",acm,first),true,is_minus)
            } else {
                (acm,str_vec,is_float,is_minus)
            }
        }
        &[] => (acm, &[],is_float,is_minus)
    }
}

//nightlyじゃないとvectorを分解できない...
fn next_token(slice: &[char]) -> (Token, &[char]) {
    match slice {
        [first, rest..] => match first {
            '\n' => next_token(rest),
            ' ' => next_token(rest),
            '=' => (Token::EQUAL, rest),
            '(' => (Token::LPAR, rest),
            ')' => (Token::RPAR, rest),
            '{' => (Token::LBRACE, rest),
            '}' => (Token::RBRACE, rest),
            '[' => (Token::LBRACKET, rest),
            ']' => (Token::RBRACKET, rest),
            ':' => (Token::COLON, rest),
            ',' => (Token::COMMA, rest),
            '+' => (Token::PLUS, rest),
            '-' => (Token::MINUS, rest),
            '*' => (Token::MUL, rest),
            '/' => (Token::DIV, rest),
            '!' => (Token::NOT,rest),
            c =>
                if c.is_numeric() || *c == '-' {
                    let (num_str, re,is_float,_) = get_num_str(slice); //moveもmutableな参照もしてないからここでslice使える
                    if is_float {
                        let num = num_str.parse::<f64>().unwrap();
                        (Token::FLOAT(num), re)
                    } else {
                        let num = num_str.parse::<i64>().unwrap();
                        (Token::INT(num), re)
                    }
                } else if *c == '\"'{
                    get_string(slice)
                } else {
                    get_keyword(slice)
                }
        },
        [] => (Token::EOF, &[])
    }
}

fn get_tokens<'a>(slice: &[char],acm: &'a mut Vec<Token>) -> &'a Vec<Token> {
    match next_token(slice) {
        (Token::EOF,_) => acm,
        (token,slice) => {
            acm.push(token);
            get_tokens(slice,acm)
        },
    }

    //stack over flow避けるなら下の書き方になるが...
    /*let mut s = slice;
    while s.len() > 0 {
        match next_token(s) {
            (Token::EOF,_) => (),
            (token,slice) => {
                s = slice;
                acm.push(token);
            },
            (token,_) => ()
        }
    }
    acm*/
}

pub fn str_to_tokens(str: &str) -> Vec<Token> {
    let str_vec = split_string(str);
    get_tokens(&str_vec,&mut vec![]).to_owned()
}

