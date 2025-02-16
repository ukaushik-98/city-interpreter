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

pub enum TokenOperators {
    ASSIGN(char),
    PLUS(char),
}

pub enum TokenDelimeters {
    COMMA(char),
    SEMICOLON(char),
    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),
}

pub enum TokenExtra<'a> {
    ILLEGAL(&'a str),
    EOF(&'a str),
}

pub enum Tokens<'a> {
    Keywords(TokenKeywords<'a>),
    Literals(TokenLiterals<'a>),
    Operators(TokenOperators),
    Delimeters(TokenDelimeters),
    Extra(TokenExtra<'a>),
}
