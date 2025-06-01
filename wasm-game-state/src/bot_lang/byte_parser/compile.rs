use chumsky::{
    Parser,
    error::Rich,
    extra,
    input::{Input, Stream, ValueInput},
    prelude::choice,
    select,
    span::SimpleSpan,
};

use super::chunk::{Chunk, OpCode};
use super::scanner::Token;

pub fn compile(src: &str) {
    let mut temp = Token::Newline; // need for Semicolon insertion
    let lexer = Token::lexer(&src)
        .spanned()
        .map(|(token, span)| match token {
            Ok(token) => (token, span),
            Err(_) => (Token::Error, span),
        })
        // auto Semicolon insertion following similar rules to golang
        .filter_map(move |x| {
            if x.0 == Token::Newline {
                return match temp {
                    Token::Ident(_)
                    | Token::Integer(_)
                    | Token::True
                    | Token::False
                    | Token::Return
                    | Token::Break
                    | Token::Float(_)
                    | Token::String(_)
                    | Token::RBrackt
                    | Token::RBrace
                    | Token::RParen => {
                        temp = Token::Semicolon;
                        Some((Token::Semicolon, x.1))
                    }
                    _ => None,
                };
            }
            temp = x.0;
            Some(x)
        });

    let token_stream = Stream::from_iter(lexer).map((0..src.len()).into(), |(t, s)| (t, s.into()));
}

pub fn compiler<'a, I>() -> impl Parser<'a, I, Chunk, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let atom = {
        // let parens = p
        //     .clone()
        //     .delimited_by(just(Token::LParen), just(Token::RParen))
        //     .map(|| Expr::Prens(Box::new(expr)));

        // let integer = select! {
        //     Token::Integer(n) => Expr::Int(n.parse().unwrap()),
        // };

        let float = select! {
            Token::Float(n) => [OpCode::Constant, ],
        };

        // let string = select! {
        //     Token::String(s) => Expr::String(s.into())
        // };
        //
        // let r#bool = select! {
        //     Token::True => Expr::Bool(true),
        //     Token::False => Expr::Bool(false)
        // };
        //
        // let nil = just(Token::Nil).to(Expr::Nil);
        //
        // let ident = select! {
        //     Token::Ident(s) => Expr::Ident(s.into())
        // };

        choice((
            // parens,
            float,
            // integer,
            // string,
            // r#bool,
            // nil,
            // ident
        ))
    }
    .pratt();
}
