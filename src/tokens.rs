struct TokenType<'a> {
    token: Tokens<'a>,
    literal: &'a str,
}

pub enum TokenKeywords<'a> {
    FUNCTION(&'a str),
    LET(&'a str),
}

pub enum TokenLiterals<'a> {
    IDENT(&'a str),
    INT(&'a str),
}

pub enum TokenOperators<'a> {
    ASSIGN(&'a str),
    PLUS(&'a str),
}

pub enum TokenDelimeters<'a> {
    COMMA(&'a str),
    SEMICOLON(&'a str),
    LPAREN(&'a str),
    RPAREN(&'a str),
    LBRACE(&'a str),
    RBRACE(&'a str),
}

pub enum TokenExtra<'a> {
    ILLEGAL(&'a str),
    EOF(&'a str),
}

pub enum Tokens<'a> {
    Keywords(TokenKeywords<'a>),
    Literals(TokenLiterals<'a>),
    Operators(TokenOperators<'a>),
    Delimeters(TokenDelimeters<'a>),
    Extra(TokenExtra<'a>),
}
