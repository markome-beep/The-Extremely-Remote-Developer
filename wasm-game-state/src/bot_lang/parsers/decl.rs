use crate::bot_lang::lexer::Token;
use chumsky::{input::ValueInput, prelude::*};

use super::expr::{Expr, parser as exprParser};

#[derive(Debug)]
pub enum Decl {
    VarDecl { ident: String, expr: Expr },
    ExprStmt(Expr),
}

pub fn parser<'a, I>() -> impl Parser<'a, I, Vec<Decl>, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let bare_var = just(Token::Let)
        .ignore_then(select! {
        Token::Ident(s) => s.into()
        })
        .then_ignore(just(Token::Semicolon))
        .map(|s| Decl::VarDecl {
            ident: s,
            expr: Expr::Nil,
        });

    let var = just(Token::Let)
        .ignore_then(select! {
        Token::Ident(s) => s.into()
        })
        .then_ignore(just(Token::Assign))
        .then(exprParser())
        .then_ignore(just(Token::Semicolon))
        .map(|(ident, expr)| Decl::VarDecl { ident, expr });

    let expr_stmt = exprParser()
        .then_ignore(just(Token::Semicolon))
        .map(|expr| Decl::ExprStmt(expr));

    //let assign = select! {
    //    Token::Ident(s) => s.into()
    //}
    //.then_ignore(just(Token::Assign))
    //.then(exprParser())
    //.then_ignore(just(Token::Semicolon))
    //.map(|(ident, expr)| Decl::Assign { ident, expr });

    choice((
        bare_var, var, expr_stmt,
        //assign
    ))
    .repeated()
    .collect()
}
