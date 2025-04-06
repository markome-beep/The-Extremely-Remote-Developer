use std::fmt;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Eq, Hash, Copy)]
#[logos(skip r"[ \t]+")]
#[logos(error = String)]
pub enum Token<'src> {
    // Num Operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("%")]
    Modulo,

    #[token("//")]
    IntDivide,

    // Bool Op
    #[token("<")]
    Less,

    #[token(">")]
    Greater,

    #[token("<=")]
    LessEq,

    #[token(">=")]
    GreaterEq,

    #[token("==")]
    Equality,

    #[token("!=")]
    NotEq,

    #[token("!")]
    #[token("not")]
    Not,

    #[token("&&")]
    #[token("and")]
    And,

    #[token("||")]
    #[token("or")]
    Or,

    // Debug Op
    #[token("$")]
    Debug,

    // Eatly Error Return
    #[token("?")]
    EarlyReturn,

    // Grouping
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBrackt,

    #[token("]")]
    RBrackt,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    // Separators
    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    // Period
    #[token(".")]
    Period,

    //
    #[token("=")]
    Assign,

    // Literals
    #[regex(r"[0-9]\.[0-9]*")]
    #[regex(r"[1-9][0-9]+\.[0-9]*")]
    #[regex(r"\.[0-9]+")]
    Float(&'src str),

    #[regex("([0-9]|[1-9][0-9]+)")]
    Integer(&'src str),

    #[regex(r#""([^"\\]|\\.)*""#, trim_quotes)]
    #[regex(r#"'([^'\\]|\\.)*'"#m, trim_quotes)]
    String(&'src str),

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Ident(&'src str),

    #[token("false")]
    False,

    #[token("true")]
    True,

    #[token("nil")]
    Nil,

    // Keywords
    #[token("fn")]
    Function,

    #[token("struct")]
    Struct,

    #[token("let")]
    Let,

    #[token("break")]
    Break,

    #[token("return")]
    Return,

    #[token("loop")]
    Loop,

    #[token("if")]
    If,

    // comments
    #[regex(r"\(\*.*\*\)", logos::skip)]
    Comment,

    Error,

    #[token("\n")]
    Newline,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Modulo => write!(f, "%"),
            Token::IntDivide => write!(f, "//"),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEq => write!(f, "<="),
            Token::GreaterEq => write!(f, ">="),
            Token::Equality => write!(f, "=="),
            Token::NotEq => write!(f, "!="),
            Token::Not => write!(f, "!"),
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Debug => write!(f, "$"),
            Token::EarlyReturn => write!(f, "?"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBrackt => write!(f, "["),
            Token::RBrackt => write!(f, "]"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Period => write!(f, "."),
            Token::Assign => write!(f, "="),
            Token::Float(s) => write!(f, "{}", s),
            Token::Integer(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::Ident(s) => write!(f, "{}", s),
            Token::False => write!(f, "false"),
            Token::True => write!(f, "true"),
            Token::Nil => write!(f, "nil"),
            Token::Function => write!(f, "fn"),
            Token::Struct => write!(f, "struct"),
            Token::Let => write!(f, "let"),
            Token::Break => write!(f, "break"),
            Token::Return => write!(f, "return"),
            Token::Loop => write!(f, "loop"),
            Token::If => write!(f, "if"),
            Token::Comment => write!(f, "<comment>"),
            Token::Error => write!(f, "<error>"),
            Token::Newline => write!(f, "<newline>"),
        }
    }
}

fn trim_quotes<'src>(lexer: &mut logos::Lexer<'src, Token<'src>>) -> &'src str {
    let slice = lexer.slice();
    &slice[1..slice.len() - 1]
}
