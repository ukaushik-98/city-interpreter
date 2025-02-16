use crate::tokens::{
    TokenDelimeters, TokenExtra, TokenKeywords, TokenLiterals, TokenOperators, Tokens,
};

fn parse_keywords<'a>(kw: TokenKeywords) -> &'a str {
    match kw {
        TokenKeywords::FUNCTION(w) => match w {
            "fn" => "fn",
            _ => unreachable!(),
        },
        TokenKeywords::LET(w) => match w {
            "let" => "let",
            _ => unreachable!(),
        },
    }
}

fn parse_literals<'a>(lt: TokenLiterals) -> &'a str {
    match lt {
        TokenLiterals::IDENT(_) => todo!(),
        TokenLiterals::INT(_) => todo!(),
    };
    ""
}

fn parse_operators<'a>(ops: TokenOperators) -> &'a str {
    match ops {
        TokenOperators::ASSIGN(_) => todo!(),
        TokenOperators::PLUS(_) => todo!(),
    };
    ""
}

fn parse_delim<'a>(dl: TokenDelimeters) -> &'a str {
    match dl {
        TokenDelimeters::COMMA(_) => todo!(),
        TokenDelimeters::SEMICOLON(_) => todo!(),
        TokenDelimeters::LPAREN(_) => todo!(),
        TokenDelimeters::RPAREN(_) => todo!(),
        TokenDelimeters::LBRACE(_) => todo!(),
        TokenDelimeters::RBRACE(_) => todo!(),
    };
    ""
}

fn parse_extra<'a>(xt: TokenExtra) -> &'a str {
    match xt {
        TokenExtra::ILLEGAL(_) => todo!(),
        TokenExtra::EOF(_) => todo!(),
    };
    ""
}

pub fn parse<'a>(token: Tokens) -> &'a str {
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
