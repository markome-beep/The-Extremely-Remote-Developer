use std::io::Cursor;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Input, Stream},
    Parser,
};
use lexer::Token;
use logos::Logos;
use parsers::decl::parser;
pub use parsers::decl::Decl;

mod lexer;
mod parsers;

pub fn gen_ast(src: &str) -> (Vec<Decl>, String) {
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

    match parser().parse(token_stream).into_result() {
        Ok(decls) => (decls, "All Good".into()),
        Err(errs) => {
            let mut error_string = Cursor::new(Vec::new());
            for err in errs {
                Report::build(ReportKind::Error, err.span().start..err.span().end)
                    .with_code(3)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(err.span().into_range())
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .write(Source::from(src), &mut error_string)
                    //.eprint(Source::from(src))
                    .unwrap();
            }
            (
                vec![],
                String::from_utf8(error_string.into_inner()).unwrap_or("RIP ERROR".into()),
            )
        }
    }
}
