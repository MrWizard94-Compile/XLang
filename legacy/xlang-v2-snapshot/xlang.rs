//! XLang Bootstrap Frontend — SOUL-compliant
//!
//! Lexer + recursive-descent AST parser + monomorphic type checker
//! with local inference for `let` bindings.
//!
//! Zero dependencies. Single file. Zero warnings under
//! `rustc --edition 2021 -W warnings`.
//!
//! # Invariants (executable where possible)
//! - Every token carries a precise Span.
//! - Function signatures are collected before bodies (mutual recursion safe).
//! - `let` without annotation is inferred; annotation must match exactly.
//! - Variables cannot be Void. Parameters cannot be Void.
//! - `for` loop variables are always Int; bounds must be Int.
//! - Call targets must be bare function names.
//! - Duplicate bindings in the same scope are rejected.
//!
//! # Deliberate limits of this bootstrap (honest)
//! - No if/else.
//! - No post-declaration assignment.
//! - No arrays, references, structs, enums, pattern matching, generics.
//! - No control-flow analysis for missing returns in non-Void functions.
//! - Environments are cloned on block entry (acceptable for bootstrap size).
//! - Named types are accepted but never defined or resolved further.
//!
//! These limits are intentional and documented so they can be closed
//! in later complete, zero-warning slices under SOUL discipline.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;

// ============================================================
// Span + Diagnostics
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    fn new(index: usize, line: usize, column: usize) -> Self {
        Self {
            index,
            line,
            column,
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub span: Span,
    pub message: String,
}

impl LexError {
    fn new<S: Into<String>>(span: Span, message: S) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lex error at {}: {}", self.span, self.message)
    }
}

impl Error for LexError {}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
}

impl ParseError {
    fn new<S: Into<String>>(span: Span, message: S) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at {}: {}", self.span, self.message)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
pub struct TypeError {
    pub span: Span,
    pub message: String,
}

impl TypeError {
    fn new<S: Into<String>>(span: Span, message: S) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "type error at {}: {}", self.span, self.message)
    }
}

impl Error for TypeError {}

#[derive(Debug)]
pub enum FrontendError {
    Lex(LexError),
    Parse(ParseError),
    Type(TypeError),
}

impl fmt::Display for FrontendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrontendError::Lex(e) => write!(f, "{e}"),
            FrontendError::Parse(e) => write!(f, "{e}"),
            FrontendError::Type(e) => write!(f, "{e}"),
        }
    }
}

impl Error for FrontendError {}

impl From<LexError> for FrontendError {
    fn from(value: LexError) -> Self {
        FrontendError::Lex(value)
    }
}

impl From<ParseError> for FrontendError {
    fn from(value: ParseError) -> Self {
        FrontendError::Parse(value)
    }
}

impl From<TypeError> for FrontendError {
    fn from(value: TypeError) -> Self {
        FrontendError::Type(value)
    }
}

// ============================================================
// Lexer
// ============================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SimpleKind {
    Ident,
    Int,
    Str,
    Let,
    Fn,
    Return,
    While,
    For,
    In,
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    Bang,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AndAnd,
    OrOr,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    DotDot,
    DotDotEq,
    Arrow,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Ident(String),
    Int(i64),
    Str(String),
    Let,
    Fn,
    Return,
    While,
    For,
    In,
    True,
    False,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    Bang,
    BangEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    AndAnd,
    OrOr,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Semicolon,
    DotDot,
    DotDotEq,
    Arrow,
    Eof,
}

impl TokenKind {
    fn simple_kind(&self) -> SimpleKind {
        match self {
            TokenKind::Ident(_) => SimpleKind::Ident,
            TokenKind::Int(_) => SimpleKind::Int,
            TokenKind::Str(_) => SimpleKind::Str,
            TokenKind::Let => SimpleKind::Let,
            TokenKind::Fn => SimpleKind::Fn,
            TokenKind::Return => SimpleKind::Return,
            TokenKind::While => SimpleKind::While,
            TokenKind::For => SimpleKind::For,
            TokenKind::In => SimpleKind::In,
            TokenKind::True => SimpleKind::True,
            TokenKind::False => SimpleKind::False,
            TokenKind::Plus => SimpleKind::Plus,
            TokenKind::Minus => SimpleKind::Minus,
            TokenKind::Star => SimpleKind::Star,
            TokenKind::Slash => SimpleKind::Slash,
            TokenKind::Percent => SimpleKind::Percent,
            TokenKind::Eq => SimpleKind::Eq,
            TokenKind::EqEq => SimpleKind::EqEq,
            TokenKind::Bang => SimpleKind::Bang,
            TokenKind::BangEq => SimpleKind::BangEq,
            TokenKind::Lt => SimpleKind::Lt,
            TokenKind::LtEq => SimpleKind::LtEq,
            TokenKind::Gt => SimpleKind::Gt,
            TokenKind::GtEq => SimpleKind::GtEq,
            TokenKind::AndAnd => SimpleKind::AndAnd,
            TokenKind::OrOr => SimpleKind::OrOr,
            TokenKind::LParen => SimpleKind::LParen,
            TokenKind::RParen => SimpleKind::RParen,
            TokenKind::LBrace => SimpleKind::LBrace,
            TokenKind::RBrace => SimpleKind::RBrace,
            TokenKind::Comma => SimpleKind::Comma,
            TokenKind::Colon => SimpleKind::Colon,
            TokenKind::Semicolon => SimpleKind::Semicolon,
            TokenKind::DotDot => SimpleKind::DotDot,
            TokenKind::DotDotEq => SimpleKind::DotDotEq,
            TokenKind::Arrow => SimpleKind::Arrow,
            TokenKind::Eof => SimpleKind::Eof,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn lex_all(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let eof = token.kind.simple_kind() == SimpleKind::Eof;
            tokens.push(token);
            if eof {
                break;
            }
        }
        Ok(tokens)
    }

    fn current_span(&self) -> Span {
        Span::new(self.pos, self.line, self.column)
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_ws_and_comments(&mut self) -> Result<(), LexError> {
        loop {
            while matches!(self.peek_char(), Some(c) if c.is_whitespace()) {
                self.next_char();
            }
            let Some('/') = self.peek_char() else {
                break;
            };
            match self.peek_next_char() {
                Some('/') => {
                    self.next_char();
                    self.next_char();
                    while let Some(c) = self.peek_char() {
                        if c == '\n' {
                            break;
                        }
                        self.next_char();
                    }
                }
                Some('*') => {
                    let start = self.current_span();
                    self.next_char();
                    self.next_char();
                    loop {
                        match self.next_char() {
                            Some('*') if self.peek_char() == Some('/') => {
                                self.next_char();
                                break;
                            }
                            Some(_) => {}
                            None => {
                                return Err(LexError::new(start, "unterminated block comment"));
                            }
                        }
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_ws_and_comments()?;
        let start = self.current_span();
        let Some(ch) = self.peek_char() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                span: start,
            });
        };

        let token = match ch {
            '(' => {
                self.next_char();
                TokenKind::LParen
            }
            ')' => {
                self.next_char();
                TokenKind::RParen
            }
            '{' => {
                self.next_char();
                TokenKind::LBrace
            }
            '}' => {
                self.next_char();
                TokenKind::RBrace
            }
            ',' => {
                self.next_char();
                TokenKind::Comma
            }
            ':' => {
                self.next_char();
                TokenKind::Colon
            }
            ';' => {
                self.next_char();
                TokenKind::Semicolon
            }
            '+' => {
                self.next_char();
                TokenKind::Plus
            }
            '-' => {
                self.next_char();
                if self.peek_char() == Some('>') {
                    self.next_char();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                self.next_char();
                TokenKind::Star
            }
            '%' => {
                self.next_char();
                TokenKind::Percent
            }
            '=' => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::EqEq
                } else {
                    TokenKind::Eq
                }
            }
            '!' => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::BangEq
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::LtEq
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                self.next_char();
                if self.peek_char() == Some('=') {
                    self.next_char();
                    TokenKind::GtEq
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                self.next_char();
                if self.peek_char() == Some('&') {
                    self.next_char();
                    TokenKind::AndAnd
                } else {
                    return Err(LexError::new(start, "single `&` is not a valid token"));
                }
            }
            '|' => {
                self.next_char();
                if self.peek_char() == Some('|') {
                    self.next_char();
                    TokenKind::OrOr
                } else {
                    return Err(LexError::new(start, "single `|` is not a valid token"));
                }
            }
            '/' => {
                self.next_char();
                TokenKind::Slash
            }
            '.' => {
                self.next_char();
                if self.peek_char() == Some('.') {
                    self.next_char();
                    if self.peek_char() == Some('=') {
                        self.next_char();
                        TokenKind::DotDotEq
                    } else {
                        TokenKind::DotDot
                    }
                } else {
                    return Err(LexError::new(start, "single `.` is not valid"));
                }
            }
            '"' => {
                return self.lex_string(start);
            }
            c if c.is_ascii_digit() => {
                return self.lex_number(start);
            }
            c if is_ident_start(c) => {
                return self.lex_ident_or_keyword(start);
            }
            _ => {
                return Err(LexError::new(
                    start,
                    format!("unexpected character `{ch}`"),
                ));
            }
        };

        Ok(Token {
            kind: token,
            span: start,
        })
    }

    fn lex_number(&mut self, start: Span) -> Result<Token, LexError> {
        let mut raw = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() || c == '_' {
                raw.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        let cleaned: String = raw.chars().filter(|c| *c != '_').collect();
        let value = cleaned
            .parse::<i64>()
            .map_err(|_| LexError::new(start, "invalid integer literal"))?;
        Ok(Token {
            kind: TokenKind::Int(value),
            span: start,
        })
    }

    fn lex_ident_or_keyword(&mut self, start: Span) -> Result<Token, LexError> {
        let mut text = String::new();
        while let Some(c) = self.peek_char() {
            if is_ident_continue(c) {
                text.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        let kind = match text.as_str() {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident(text),
        };
        Ok(Token {
            kind,
            span: start,
        })
    }

    fn lex_string(&mut self, start: Span) -> Result<Token, LexError> {
        self.next_char(); // opening quote
        let mut out = String::new();
        while let Some(c) = self.next_char() {
            match c {
                '"' => {
                    return Ok(Token {
                        kind: TokenKind::Str(out),
                        span: start,
                    });
                }
                '\\' => {
                    let Some(esc) = self.next_char() else {
                        return Err(LexError::new(start, "unterminated string literal"));
                    };
                    let translated = match esc {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        '"' => '"',
                        '\\' => '\\',
                        '0' => '\0',
                        _ => {
                            return Err(LexError::new(
                                start,
                                format!("unknown escape sequence `\\{esc}`"),
                            ));
                        }
                    };
                    out.push(translated);
                }
                '\n' => {
                    return Err(LexError::new(start, "unterminated string literal"));
                }
                other => out.push(other),
            }
        }
        Err(LexError::new(start, "unterminated string literal"))
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

// ============================================================
// AST
// ============================================================

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Item {
    Function(FunctionDecl),
    Stmt(Stmt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: TypeName,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    pub name: String,
    pub ty: TypeName,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    fn new(kind: ExprKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind {
    Int(i64),
    Bool(bool),
    Str(String),
    Var(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Group(Box<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Neg => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Mod => write!(f, "%"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Neq => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Lte => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Gte => write!(f, ">="),
            BinaryOp::And => write!(f, "&&"),
            BinaryOp::Or => write!(f, "||"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    fn new(kind: StmtKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    Let {
        name: String,
        ty: Option<TypeName>,
        value: Expr,
    },
    Return(Option<Expr>),
    Expr(Expr),
    Block(Vec<Stmt>),
    While {
        cond: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        inclusive: bool,
        body: Vec<Stmt>,
    },
}

/// Surface type names written by the programmer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeName {
    Int,
    Bool,
    Str,
    Void,
    Named(String),
}

impl fmt::Display for TypeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeName::Int => write!(f, "Int"),
            TypeName::Bool => write!(f, "Bool"),
            TypeName::Str => write!(f, "String"),
            TypeName::Void => write!(f, "Void"),
            TypeName::Named(name) => write!(f, "{name}"),
        }
    }
}

/// Resolved types used by the type checker.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    Str,
    Void,
    Named(String),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::Str => write!(f, "String"),
            Type::Void => write!(f, "Void"),
            Type::Named(name) => write!(f, "{name}"),
        }
    }
}

// ============================================================
// Parser
// ============================================================

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut items = Vec::new();
        while !self.check(SimpleKind::Eof) {
            if self.check(SimpleKind::Fn) {
                items.push(Item::Function(self.parse_function()?));
            } else {
                items.push(Item::Stmt(self.parse_stmt()?));
            }
        }
        Ok(Program { items })
    }

    fn parse_function(&mut self) -> Result<FunctionDecl, ParseError> {
        let fn_tok = self.expect(SimpleKind::Fn, "expected `fn`")?;
        let (name, _) = self.expect_ident()?;
        self.expect(SimpleKind::LParen, "expected `(` after function name")?;

        let mut params = Vec::new();
        if !self.check(SimpleKind::RParen) {
            loop {
                let (param_name, param_span) = self.expect_ident()?;
                self.expect(SimpleKind::Colon, "expected `:` after parameter name")?;
                let ty = self.parse_type_name()?;
                params.push(Param {
                    name: param_name,
                    ty,
                    span: param_span,
                });
                if self.consume_if(SimpleKind::Comma).is_some() {
                    if self.check(SimpleKind::RParen) {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        self.expect(SimpleKind::RParen, "expected `)` after parameter list")?;
        self.expect(SimpleKind::Arrow, "expected `->` after parameter list")?;
        let return_type = self.parse_type_name()?;
        let (body, _) = self.parse_block()?;

        Ok(FunctionDecl {
            name,
            params,
            return_type,
            body,
            span: fn_tok.span,
        })
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.check(SimpleKind::Let) {
            return self.parse_let_stmt();
        }
        if self.check(SimpleKind::Return) {
            return self.parse_return_stmt();
        }
        if self.check(SimpleKind::While) {
            return self.parse_while_stmt();
        }
        if self.check(SimpleKind::For) {
            return self.parse_for_stmt();
        }
        if self.check(SimpleKind::LBrace) {
            let (stmts, span) = self.parse_block()?;
            return Ok(Stmt::new(StmtKind::Block(stmts), span));
        }

        let expr = self.parse_expr()?;
        let span = expr.span;
        self.expect(SimpleKind::Semicolon, "expected `;` after expression")?;
        Ok(Stmt::new(StmtKind::Expr(expr), span))
    }

    fn parse_let_stmt(&mut self) -> Result<Stmt, ParseError> {
        let let_tok = self.expect(SimpleKind::Let, "expected `let`")?;
        let (name, _) = self.expect_ident()?;
        let ty = if self.consume_if(SimpleKind::Colon).is_some() {
            Some(self.parse_type_name()?)
        } else {
            None
        };
        self.expect(SimpleKind::Eq, "expected `=` in variable declaration")?;
        let value = self.parse_expr()?;
        self.expect(
            SimpleKind::Semicolon,
            "expected `;` after variable declaration",
        )?;
        Ok(Stmt::new(
            StmtKind::Let { name, ty, value },
            let_tok.span,
        ))
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let ret_tok = self.expect(SimpleKind::Return, "expected `return`")?;
        if self.check(SimpleKind::Semicolon) {
            self.advance();
            return Ok(Stmt::new(StmtKind::Return(None), ret_tok.span));
        }
        let expr = self.parse_expr()?;
        self.expect(
            SimpleKind::Semicolon,
            "expected `;` after return expression",
        )?;
        Ok(Stmt::new(StmtKind::Return(Some(expr)), ret_tok.span))
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        let tok = self.expect(SimpleKind::While, "expected `while`")?;
        let cond = self.parse_expr()?;
        let (body, _) = self.parse_block()?;
        Ok(Stmt::new(StmtKind::While { cond, body }, tok.span))
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, ParseError> {
        let tok = self.expect(SimpleKind::For, "expected `for`")?;
        let (var, _) = self.expect_ident()?;
        self.expect(SimpleKind::In, "expected `in` in for-loop")?;
        let start = self.parse_expr()?;
        let inclusive = if self.consume_if(SimpleKind::DotDotEq).is_some() {
            true
        } else {
            self.expect(SimpleKind::DotDot, "expected `..` or `..=` in for-loop")?;
            false
        };
        let end = self.parse_expr()?;
        let (body, _) = self.parse_block()?;
        Ok(Stmt::new(
            StmtKind::For {
                var,
                start,
                end,
                inclusive,
                body,
            },
            tok.span,
        ))
    }

    fn parse_block(&mut self) -> Result<(Vec<Stmt>, Span), ParseError> {
        let lbrace = self.expect(SimpleKind::LBrace, "expected `{`")?;
        let start_span = lbrace.span;
        let mut stmts = Vec::new();
        while !self.check(SimpleKind::RBrace) {
            if self.check(SimpleKind::Eof) {
                return Err(ParseError::new(start_span, "unterminated block"));
            }
            stmts.push(self.parse_stmt()?);
        }
        self.expect(SimpleKind::RBrace, "expected `}` to close block")?;
        Ok((stmts, start_span))
    }

    fn parse_type_name(&mut self) -> Result<TypeName, ParseError> {
        let (name, _) = self.expect_ident()?;
        let ty = match name.as_str() {
            "Int" => TypeName::Int,
            "Bool" => TypeName::Bool,
            "Str" | "String" => TypeName::Str,
            "Void" => TypeName::Void,
            other => TypeName::Named(other.to_string()),
        };
        Ok(ty)
    }

    // ---------------- Expressions ----------------

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_and()?;
        while let Some(op) = self.consume_if(SimpleKind::OrOr) {
            let rhs = self.parse_and()?;
            expr = Expr::new(
                ExprKind::Binary {
                    op: BinaryOp::Or,
                    left: Box::new(expr),
                    right: Box::new(rhs),
                },
                op.span,
            );
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_equality()?;
        while let Some(op) = self.consume_if(SimpleKind::AndAnd) {
            let rhs = self.parse_equality()?;
            expr = Expr::new(
                ExprKind::Binary {
                    op: BinaryOp::And,
                    left: Box::new(expr),
                    right: Box::new(rhs),
                },
                op.span,
            );
        }
        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_comparison()?;
        loop {
            if let Some(op) = self.consume_if(SimpleKind::EqEq) {
                let rhs = self.parse_comparison()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Eq,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::BangEq) {
                let rhs = self.parse_comparison()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Neq,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_term()?;
        loop {
            if let Some(op) = self.consume_if(SimpleKind::Lt) {
                let rhs = self.parse_term()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Lt,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::LtEq) {
                let rhs = self.parse_term()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Lte,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::Gt) {
                let rhs = self.parse_term()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Gt,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::GtEq) {
                let rhs = self.parse_term()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Gte,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_factor()?;
        loop {
            if let Some(op) = self.consume_if(SimpleKind::Plus) {
                let rhs = self.parse_factor()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Add,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::Minus) {
                let rhs = self.parse_factor()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Sub,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_unary()?;
        loop {
            if let Some(op) = self.consume_if(SimpleKind::Star) {
                let rhs = self.parse_unary()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Mul,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::Slash) {
                let rhs = self.parse_unary()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Div,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else if let Some(op) = self.consume_if(SimpleKind::Percent) {
                let rhs = self.parse_unary()?;
                expr = Expr::new(
                    ExprKind::Binary {
                        op: BinaryOp::Mod,
                        left: Box::new(expr),
                        right: Box::new(rhs),
                    },
                    op.span,
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(op) = self.consume_if(SimpleKind::Bang) {
            let rhs = self.parse_unary()?;
            return Ok(Expr::new(
                ExprKind::Unary {
                    op: UnaryOp::Not,
                    expr: Box::new(rhs),
                },
                op.span,
            ));
        }
        if let Some(op) = self.consume_if(SimpleKind::Minus) {
            let rhs = self.parse_unary()?;
            return Ok(Expr::new(
                ExprKind::Unary {
                    op: UnaryOp::Neg,
                    expr: Box::new(rhs),
                },
                op.span,
            ));
        }
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary()?;
        loop {
            if self.consume_if(SimpleKind::LParen).is_some() {
                let mut args = Vec::new();
                if !self.check(SimpleKind::RParen) {
                    loop {
                        args.push(self.parse_expr()?);
                        if self.consume_if(SimpleKind::Comma).is_some() {
                            if self.check(SimpleKind::RParen) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                self.expect(SimpleKind::RParen, "expected `)` after call arguments")?;
                let span = expr.span;
                expr = Expr::new(
                    ExprKind::Call {
                        callee: Box::new(expr),
                        args,
                    },
                    span,
                );
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Int(v) => {
                self.advance();
                Ok(Expr::new(ExprKind::Int(v), tok.span))
            }
            TokenKind::Str(s) => {
                self.advance();
                Ok(Expr::new(ExprKind::Str(s), tok.span))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::new(ExprKind::Bool(true), tok.span))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::new(ExprKind::Bool(false), tok.span))
            }
            TokenKind::Ident(name) => {
                self.advance();
                Ok(Expr::new(ExprKind::Var(name), tok.span))
            }
            TokenKind::LParen => {
                let open = self.advance();
                let inner = self.parse_expr()?;
                self.expect(
                    SimpleKind::RParen,
                    "expected `)` after grouped expression",
                )?;
                Ok(Expr::new(ExprKind::Group(Box::new(inner)), open.span))
            }
            _ => Err(ParseError::new(
                tok.span,
                format!("unexpected token in expression: {:?}", tok.kind),
            )),
        }
    }

    // ---------------- helpers ----------------

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn check(&self, kind: SimpleKind) -> bool {
        self.peek().kind.simple_kind() == kind
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek().clone();
        if self.pos + 1 < self.tokens.len() {
            self.pos += 1;
        }
        tok
    }

    fn consume_if(&mut self, kind: SimpleKind) -> Option<Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: SimpleKind, message: &str) -> Result<Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let got = self.peek().clone();
            Err(ParseError::new(
                got.span,
                format!("{message}; found {:?}", got.kind),
            ))
        }
    }

    fn expect_ident(&mut self) -> Result<(String, Span), ParseError> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::Ident(name) => {
                self.advance();
                Ok((name, tok.span))
            }
            _ => Err(ParseError::new(
                tok.span,
                format!("expected identifier, found {:?}", tok.kind),
            )),
        }
    }
}

// ============================================================
// Type Checker
// ============================================================

#[derive(Clone, Debug)]
struct FunctionSig {
    params: Vec<Type>,
    return_type: Type,
}

#[derive(Clone, Debug)]
struct TypeEnv {
    scopes: Vec<HashMap<String, Type>>,
}

impl TypeEnv {
    fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn lookup(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }

    fn lookup_current(&self, name: &str) -> Option<&Type> {
        self.scopes.last().and_then(|scope| scope.get(name))
    }

    fn insert(&mut self, name: String, ty: Type, span: Span) -> Result<(), TypeError> {
        let current = self
            .scopes
            .last_mut()
            .expect("type environment always has one scope");
        if current.contains_key(&name) {
            return Err(TypeError::new(
                span,
                format!("duplicate binding `{name}` in the same scope"),
            ));
        }
        current.insert(name, ty);
        Ok(())
    }
}

fn resolve_type_name(ty: &TypeName) -> Type {
    match ty {
        TypeName::Int => Type::Int,
        TypeName::Bool => Type::Bool,
        TypeName::Str => Type::Str,
        TypeName::Void => Type::Void,
        TypeName::Named(name) => Type::Named(name.clone()),
    }
}

pub fn check_program(program: &Program) -> Result<(), TypeError> {
    let mut functions: HashMap<String, FunctionSig> = HashMap::new();

    // Pass 1: collect all function signatures (supports mutual recursion).
    for item in &program.items {
        if let Item::Function(func) = item {
            if functions.contains_key(&func.name) {
                return Err(TypeError::new(
                    func.span,
                    format!("duplicate function `{}`", func.name),
                ));
            }
            let mut params = Vec::new();
            for param in &func.params {
                let ty = resolve_type_name(&param.ty);
                if ty == Type::Void {
                    return Err(TypeError::new(
                        param.span,
                        "function parameters cannot have type Void",
                    ));
                }
                params.push(ty);
            }
            let return_type = resolve_type_name(&func.return_type);
            functions.insert(
                func.name.clone(),
                FunctionSig {
                    params,
                    return_type,
                },
            );
        }
    }

    // Pass 2: check top-level statements and function bodies.
    let mut globals = TypeEnv::new();
    for item in &program.items {
        match item {
            Item::Stmt(stmt) => {
                check_stmt(&functions, &mut globals, stmt, None)?;
            }
            Item::Function(func) => {
                check_function(&functions, &globals, func)?;
            }
        }
    }
    Ok(())
}

fn check_function(
    functions: &HashMap<String, FunctionSig>,
    globals: &TypeEnv,
    func: &FunctionDecl,
) -> Result<(), TypeError> {
    if globals.lookup_current(&func.name).is_some() {
        return Err(TypeError::new(
            func.span,
            format!(
                "function `{}` conflicts with an existing global variable",
                func.name
            ),
        ));
    }

    let sig = functions.get(&func.name).expect("function sig must exist");
    let mut env = globals.clone();
    env.push_scope();

    for (param, ty) in func.params.iter().zip(sig.params.iter()) {
        if functions.contains_key(&param.name) {
            return Err(TypeError::new(
                param.span,
                format!(
                    "`{}` is a function name and cannot be used as a parameter",
                    param.name
                ),
            ));
        }
        env.insert(param.name.clone(), ty.clone(), param.span)?;
    }

    for stmt in &func.body {
        check_stmt(functions, &mut env, stmt, Some(&sig.return_type))?;
    }
    Ok(())
}

fn check_stmt(
    functions: &HashMap<String, FunctionSig>,
    env: &mut TypeEnv,
    stmt: &Stmt,
    current_return: Option<&Type>,
) -> Result<(), TypeError> {
    match &stmt.kind {
        StmtKind::Let { name, ty, value } => {
            if functions.contains_key(name) {
                return Err(TypeError::new(
                    stmt.span,
                    format!("`{name}` is a function name and cannot be used as a variable"),
                ));
            }
            let value_ty = check_expr(functions, env, value)?;
            let final_ty = match ty {
                Some(annot) => {
                    let expected = resolve_type_name(annot);
                    if expected != value_ty {
                        return Err(TypeError::new(
                            value.span,
                            format!(
                                "type mismatch in binding `{name}`: expected `{expected}`, found `{value_ty}`"
                            ),
                        ));
                    }
                    expected
                }
                None => {
                    if value_ty == Type::Void {
                        return Err(TypeError::new(
                            value.span,
                            "cannot infer type `Void` for a variable",
                        ));
                    }
                    value_ty
                }
            };
            if final_ty == Type::Void {
                return Err(TypeError::new(stmt.span, "variables cannot have type Void"));
            }
            env.insert(name.clone(), final_ty, stmt.span)?;
        }
        StmtKind::Return(expr) => {
            let expected = current_return.ok_or_else(|| {
                TypeError::new(stmt.span, "return statement outside of a function")
            })?;
            match expr {
                Some(e) => {
                    let got = check_expr(functions, env, e)?;
                    if expected != &got {
                        return Err(TypeError::new(
                            e.span,
                            format!("return type mismatch: expected `{expected}`, found `{got}`"),
                        ));
                    }
                }
                None => {
                    if expected != &Type::Void {
                        return Err(TypeError::new(
                            stmt.span,
                            format!(
                                "missing return expression; function returns `{expected}`"
                            ),
                        ));
                    }
                }
            }
        }
        StmtKind::Expr(expr) => {
            let _ = check_expr(functions, env, expr)?;
        }
        StmtKind::Block(stmts) => {
            let mut inner = env.clone();
            inner.push_scope();
            for s in stmts {
                check_stmt(functions, &mut inner, s, current_return)?;
            }
        }
        StmtKind::While { cond, body } => {
            let cond_ty = check_expr(functions, env, cond)?;
            if cond_ty != Type::Bool {
                return Err(TypeError::new(
                    cond.span,
                    format!("while condition must be `Bool`, found `{cond_ty}`"),
                ));
            }
            let mut inner = env.clone();
            inner.push_scope();
            for s in body {
                check_stmt(functions, &mut inner, s, current_return)?;
            }
        }
        StmtKind::For {
            var,
            start,
            end,
            inclusive: _,
            body,
        } => {
            if functions.contains_key(var) {
                return Err(TypeError::new(
                    stmt.span,
                    format!("`{var}` is a function name and cannot be used as a loop variable"),
                ));
            }
            let start_ty = check_expr(functions, env, start)?;
            let end_ty = check_expr(functions, env, end)?;
            if start_ty != Type::Int || end_ty != Type::Int {
                return Err(TypeError::new(
                    stmt.span,
                    format!(
                        "for-loop bounds must be `Int`; found `{start_ty}` and `{end_ty}`"
                    ),
                ));
            }
            let mut inner = env.clone();
            inner.push_scope();
            inner.insert(var.clone(), Type::Int, stmt.span)?;
            for s in body {
                check_stmt(functions, &mut inner, s, current_return)?;
            }
        }
    }
    Ok(())
}

fn check_expr(
    functions: &HashMap<String, FunctionSig>,
    env: &TypeEnv,
    expr: &Expr,
) -> Result<Type, TypeError> {
    match &expr.kind {
        ExprKind::Int(_) => Ok(Type::Int),
        ExprKind::Bool(_) => Ok(Type::Bool),
        ExprKind::Str(_) => Ok(Type::Str),
        ExprKind::Var(name) => {
            if let Some(ty) = env.lookup(name) {
                Ok(ty)
            } else if functions.contains_key(name) {
                Err(TypeError::new(
                    expr.span,
                    format!("`{name}` is a function; use `{name}()` to call it"),
                ))
            } else {
                Err(TypeError::new(
                    expr.span,
                    format!("unknown identifier `{name}`"),
                ))
            }
        }
        ExprKind::Group(inner) => check_expr(functions, env, inner),
        ExprKind::Unary { op, expr: inner } => {
            let ty = check_expr(functions, env, inner)?;
            match op {
                UnaryOp::Neg => {
                    if ty != Type::Int {
                        return Err(TypeError::new(
                            expr.span,
                            format!("unary `-` expects `Int`, found `{ty}`"),
                        ));
                    }
                    Ok(Type::Int)
                }
                UnaryOp::Not => {
                    if ty != Type::Bool {
                        return Err(TypeError::new(
                            expr.span,
                            format!("unary `!` expects `Bool`, found `{ty}`"),
                        ));
                    }
                    Ok(Type::Bool)
                }
            }
        }
        ExprKind::Binary { op, left, right } => {
            let lhs = check_expr(functions, env, left)?;
            let rhs = check_expr(functions, env, right)?;
            match op {
                BinaryOp::Add
                | BinaryOp::Sub
                | BinaryOp::Mul
                | BinaryOp::Div
                | BinaryOp::Mod => {
                    if lhs != Type::Int || rhs != Type::Int {
                        return Err(TypeError::new(
                            expr.span,
                            format!(
                                "operator `{op}` expects `Int` operands, found `{lhs}` and `{rhs}`"
                            ),
                        ));
                    }
                    Ok(Type::Int)
                }
                BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
                    if lhs != Type::Int || rhs != Type::Int {
                        return Err(TypeError::new(
                            expr.span,
                            format!(
                                "operator `{op}` expects `Int` operands, found `{lhs}` and `{rhs}`"
                            ),
                        ));
                    }
                    Ok(Type::Bool)
                }
                BinaryOp::And | BinaryOp::Or => {
                    if lhs != Type::Bool || rhs != Type::Bool {
                        return Err(TypeError::new(
                            expr.span,
                            format!(
                                "operator `{op}` expects `Bool` operands, found `{lhs}` and `{rhs}`"
                            ),
                        ));
                    }
                    Ok(Type::Bool)
                }
                BinaryOp::Eq | BinaryOp::Neq => {
                    if lhs == Type::Void || rhs == Type::Void {
                        return Err(TypeError::new(
                            expr.span,
                            format!("cannot compare `Void` values with `{op}`"),
                        ));
                    }
                    if lhs != rhs {
                        return Err(TypeError::new(
                            expr.span,
                            format!(
                                "operator `{op}` requires both sides to have the same type, found `{lhs}` and `{rhs}`"
                            ),
                        ));
                    }
                    Ok(Type::Bool)
                }
            }
        }
        ExprKind::Call { callee, args } => {
            let name = match &callee.kind {
                ExprKind::Var(name) => name,
                _ => {
                    return Err(TypeError::new(
                        callee.span,
                        "call target must be a named function",
                    ));
                }
            };
            let sig = functions.get(name).ok_or_else(|| {
                TypeError::new(callee.span, format!("unknown function `{name}`"))
            })?;
            if args.len() != sig.params.len() {
                return Err(TypeError::new(
                    expr.span,
                    format!(
                        "function `{name}` expects {} arguments, found {}",
                        sig.params.len(),
                        args.len()
                    ),
                ));
            }
            for (arg, expected) in args.iter().zip(sig.params.iter()) {
                let got = check_expr(functions, env, arg)?;
                if &got != expected {
                    return Err(TypeError::new(
                        arg.span,
                        format!(
                            "argument type mismatch in call to `{name}`: expected `{expected}`, found `{got}`"
                        ),
                    ));
                }
            }
            Ok(sig.return_type.clone())
        }
    }
}

// ============================================================
// Convenience entry point
// ============================================================

pub fn compile_source(source: &str) -> Result<Program, FrontendError> {
    let tokens = Lexer::new(source).lex_all()?;
    let program = Parser::new(tokens).parse_program()?;
    check_program(&program)?;
    Ok(program)
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_ok(source: &str) {
        compile_source(source).expect("expected success");
    }

    fn assert_err(source: &str) {
        assert!(compile_source(source).is_err(), "expected error");
    }

    #[test]
    fn empty_program() {
        assert_ok("");
    }

    #[test]
    fn simple_let_inference() {
        assert_ok("let x = 42;");
    }

    #[test]
    fn annotated_let() {
        assert_ok("let x: Int = 10;");
    }

    #[test]
    fn type_mismatch_rejected() {
        assert_err("let x: Bool = 1;");
    }

    #[test]
    fn function_and_call() {
        assert_ok(
            r#"
            fn add(a: Int, b: Int) -> Int {
                return a + b;
            }
            let t = add(1, 2);
            "#,
        );
    }

    #[test]
    fn wrong_arg_count() {
        assert_err(
            r#"
            fn add(a: Int, b: Int) -> Int { return a + b; }
            let t = add(1);
            "#,
        );
    }

    #[test]
    fn while_requires_bool() {
        assert_err("while 1 { }");
        assert_ok("while true { }");
    }

    #[test]
    fn for_bounds_must_be_int() {
        assert_err("for i in true .. 10 { }");
        assert_ok("for i in 0 .. 10 { let x = i; }");
    }

    #[test]
    fn void_variable_rejected() {
        assert_err(
            r#"
            fn nop() -> Void { return; }
            let x = nop();
            "#,
        );
    }

    #[test]
    fn duplicate_binding() {
        assert_err("let x = 1; let x = 2;");
    }

    #[test]
    fn unknown_identifier() {
        assert_err("let x = y;");
    }

    #[test]
    fn full_demo() {
        assert_ok(
            r#"
            fn add(a: Int, b: Int) -> Int {
                let sum = a + b;
                return sum;
            }
            fn noop() -> Void {
                return;
            }
            let base = 10;
            let total: Int = add(base, 32);
            let inferred = total + 1;
            for i in 0 .. 4 {
                let value = add(i, base);
            }
            "#,
        );
    }
}

// ============================================================
// Demo entry point
// ============================================================

fn main() {
    let source = r#"
fn add(a: Int, b: Int) -> Int {
    let sum = a + b;
    return sum;
}

fn noop() -> Void {
    return;
}

let base = 10;
let total: Int = add(base, 32);
let inferred = total + 1;

for i in 0 .. 4 {
    let value = add(i, base);
}
"#;

    match compile_source(source) {
        Ok(program) => {
            println!("Parsed and type-checked successfully.");
            println!("{:#?}", program);
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
