#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Ident(String), // Identifier
    Number(i64),   // Number literal
    Str(String),   // String literal
    Symbol(char),  // Single-character symbol
    Arrow,         // ->
    Extern,        // "extern"
    Typedef,       // "typedef"
    Int,           // "int"
    Char,          // "char"
    Void,          // "void"
    Struct,        // "struct"
    Bool,          // "_Bool"
    If,            // "if"
    Else,          // "else"
    For,           // "for"
    Do,            // "do"
    While,         // "while"
    Switch,        // "switch"
    Case,          // "case"
    Break,         // "break"
    Continue,      // "continue"
    Eq,            // ==
    Ne,            // !=
    Le,            // <=
    Ge,            // >=
    LogOr,         // ||
    LogAnd,        // &&
    Shl,           // <<
    Shr,           // >>
    Inc,           // ++
    Dec,           // --
    MulEq,         // *=
    DivEq,         // /=
    ModEq,         // %=
    AddEq,         // +=
    SubEq,         // -=
    ShlEq,         // <<=
    ShrEq,         // >>=
    AndEq,         // &=
    XorEq,         // ^=
    OrEq,          // |=
    Return,        // "return"
    Sizeof,        // "sizeof"
    Alignof,       // "_Alignof"
    Typeof,        // "typeof"
    Param,         // Function-like macro parameter
    Eof,           // End marker
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Span<'a>,
}

#[derive(Debug, Clone)]
pub struct Span<'a> {
    pub file_name: &'a str,
    pub file_content: &'a str,
    pub start: usize,
    pub end: usize,
}

impl<'a> Token<'a> {
    pub fn from_str(s: &str) -> Option<TokenKind> {
        match s {
            "<<=" => Some(TokenKind::ShlEq),
            ">>=" => Some(TokenKind::ShrEq),
            "!=" => Some(TokenKind::Ne),
            "&&" => Some(TokenKind::LogAnd),
            "++" => Some(TokenKind::Inc),
            "--" => Some(TokenKind::Dec),
            "->" => Some(TokenKind::Arrow),
            "<<" => Some(TokenKind::Shl),
            "<=" => Some(TokenKind::Le),
            "==" => Some(TokenKind::Eq),
            ">=" => Some(TokenKind::Ge),
            ">>" => Some(TokenKind::Shr),
            "||" => Some(TokenKind::LogOr),
            "*=" => Some(TokenKind::MulEq),
            "/=" => Some(TokenKind::DivEq),
            "%=" => Some(TokenKind::ModEq),
            "+=" => Some(TokenKind::AddEq),
            "-=" => Some(TokenKind::SubEq),
            "&=" => Some(TokenKind::AndEq),
            "^=" => Some(TokenKind::XorEq),
            "|=" => Some(TokenKind::OrEq),
            _ => None,
        }
    }

    pub fn from_char(c: char) -> Option<TokenKind> {
        match c {
            '+' => Some(TokenKind::Symbol(c)),
            '-' => Some(TokenKind::Symbol(c)),
            '*' => Some(TokenKind::Symbol(c)),
            '/' => Some(TokenKind::Symbol(c)),
            '%' => Some(TokenKind::Symbol(c)),
            '<' => Some(TokenKind::Symbol(c)),
            '>' => Some(TokenKind::Symbol(c)),
            '&' => Some(TokenKind::Symbol(c)),
            '^' => Some(TokenKind::Symbol(c)),
            '|' => Some(TokenKind::Symbol(c)),
            '!' => Some(TokenKind::Symbol(c)),
            '~' => Some(TokenKind::Symbol(c)),
            '=' => Some(TokenKind::Symbol(c)),
            '(' => Some(TokenKind::Symbol(c)),
            ')' => Some(TokenKind::Symbol(c)),
            '{' => Some(TokenKind::Symbol(c)),
            '}' => Some(TokenKind::Symbol(c)),
            '[' => Some(TokenKind::Symbol(c)),
            ']' => Some(TokenKind::Symbol(c)),
            ';' => Some(TokenKind::Symbol(c)),
            ',' => Some(TokenKind::Symbol(c)),
            '.' => Some(TokenKind::Symbol(c)),
            ':' => Some(TokenKind::Symbol(c)),
            _ => None,
        }
    }
}
