use crate::bot_lang::lexer::Token;
use chumsky::{input::ValueInput, prelude::*};

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum Expr {
    // literal.
    Int(i32),
    Float(f64),
    String(String),
    Bool(bool),
    Ident(String),
    Nil,
    Prens(Box<Expr>),

    // Unary minus.
    Neg(Box<Expr>),
    Not(Box<Expr>),
    Debug(SimpleSpan, Box<Expr>),

    // Binary operators
    // Sum
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),

    // Product
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    IntDiv(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),

    // Comparison
    GrEq(Box<Expr>, Box<Expr>),
    Gr(Box<Expr>, Box<Expr>),
    LeEq(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),

    // Equality
    Eq(Box<Expr>, Box<Expr>),
    NotEq(Box<Expr>, Box<Expr>),

    // Assignment
    Assignment(Box<Expr>, Box<Expr>),
}

pub fn parser<'a, I>() -> impl Parser<'a, I, Expr, extra::Err<Rich<'a, Token<'a>>>>
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    recursive(|p| {
        let atom = {
            let parens = p
                .clone()
                .delimited_by(just(Token::LParen), just(Token::RParen))
                .map(|expr| Expr::Prens(Box::new(expr)));

            let integer = select! {
                Token::Integer(n) => Expr::Int(n.parse().unwrap()),
            };

            let float = select! {
                Token::Float(n) => Expr::Float(n.parse().unwrap()),
            };

            let string = select! {
                Token::String(s) => Expr::String(s.into())
            };

            let r#bool = select! {
                Token::True => Expr::Bool(true),
                Token::False => Expr::Bool(false)
            };

            let nil = just(Token::Nil).to(Expr::Nil);

            let ident = select! {
                Token::Ident(s) => Expr::Ident(s.into())
            };

            choice((parens, float, integer, string, r#bool, nil, ident))
        };
        let debug = atom
            .clone()
            .then_ignore(just(Token::Debug))
            .map_with(|expr, extra| Expr::Debug(extra.span(), Box::new(expr)))
            .or(atom);

        let unary = just(Token::Minus)
            .or(just(Token::Not))
            .repeated()
            .foldr(debug, |op, rhs| match op {
                Token::Not => Expr::Not(Box::new(rhs)),
                Token::Minus => Expr::Neg(Box::new(rhs)),
                _ => unreachable!(),
            });

        let product = unary.clone().foldl(
            choice((
                just(Token::Modulo),
                just(Token::Multiply),
                just(Token::Divide),
                just(Token::IntDivide),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Multiply => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                Token::Divide => Expr::Div(Box::new(lhs), Box::new(rhs)),
                Token::IntDivide => Expr::IntDiv(Box::new(lhs), Box::new(rhs)),
                Token::Modulo => Expr::Mod(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let sum = product.clone().foldl(
            choice((just(Token::Plus), just(Token::Minus)))
                .then(product)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Plus => Expr::Add(Box::new(lhs), Box::new(rhs)),
                Token::Minus => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let comparison = sum.clone().foldl(
            choice((
                just(Token::Greater),
                just(Token::GreaterEq),
                just(Token::Less),
                just(Token::LessEq),
            ))
            .then(sum)
            .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Greater => Expr::Gr(Box::new(lhs), Box::new(rhs)),
                Token::GreaterEq => Expr::GrEq(Box::new(lhs), Box::new(rhs)),
                Token::Less => Expr::Le(Box::new(lhs), Box::new(rhs)),
                Token::LessEq => Expr::LeEq(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        let equality = comparison.clone().foldl(
            choice((just(Token::Equality), just(Token::NotEq)))
                .then(comparison)
                .repeated(),
            |lhs, (op, rhs)| match op {
                Token::Equality => Expr::Eq(Box::new(lhs), Box::new(rhs)),
                Token::NotEq => Expr::NotEq(Box::new(lhs), Box::new(rhs)),
                _ => unreachable!(),
            },
        );

        equality
    })
}
