#[derive(Clone,Debug)]
pub enum Token {
    STRING(String),
    INT(i64),
    //FLOAT(f64),
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COLON,
    COMMA,
    NULL,
    BOOLEAN(bool),
    EOF
}
