use crate::tokens::{
    TokenDelimeters, TokenExtra, TokenKeywords, TokenLiterals, TokenOperators, Tokens,
};

fn match_known_val<T>(w: T, l: T) -> T {
    match w {
        l => l,
        _ => unreachable!(),
    }
}

fn parse_keywords<'a>(kw: TokenKeywords<'a>) -> &'a str {
    match kw {
        TokenKeywords::FUNCTION(w) => match_known_val(w, "fn"),
        TokenKeywords::LET(w) => match_known_val(w, "let"),
    }
}

fn parse_literals<'a>(lt: TokenLiterals<'a>) -> &'a str {
    match lt {
        TokenLiterals::IDENT(w) => match_known_val(w, "IDENT"),
        TokenLiterals::INT(w) => match_known_val(w, "INT"),
    }
}

fn parse_operators<'a>(ops: TokenOperators<'a>) -> &'a str {
    match ops {
        TokenOperators::ASSIGN(w) => match_known_val(w, "="),
        TokenOperators::PLUS(w) => match_known_val(w, "+"),
    }
}

fn parse_delim<'a>(dl: TokenDelimeters<'a>) -> &'a str {
    match dl {
        TokenDelimeters::COMMA(w) => match_known_val(w, ","),
        TokenDelimeters::SEMICOLON(w) => match_known_val(w, ";"),
        TokenDelimeters::LPAREN(w) => match_known_val(w, "("),
        TokenDelimeters::RPAREN(w) => match_known_val(w, ")"),
        TokenDelimeters::LBRACE(w) => match_known_val(w, "{"),
        TokenDelimeters::RBRACE(w) => match_known_val(w, "}"),
    }
}

fn parse_extra<'a>(xt: TokenExtra<'a>) -> &'a str {
    match xt {
        TokenExtra::ILLEGAL(w) => match_known_val(w, "ILLEGAL"),
        TokenExtra::EOF(w) => match_known_val(w, "EOF"),
    }
}

pub fn parse<'a>(token: Tokens<'a>) -> &'a str {
    match token {
        Tokens::Keywords(kw) => parse_keywords(kw),
        Tokens::Literals(lt) => parse_literals(lt),
        Tokens::Operators(ops) => parse_operators(ops),
        Tokens::Delimeters(dl) => parse_delim(dl),
        Tokens::Extra(xt) => parse_extra(xt),
    }
}

#[test]
fn lexer_test() {}
