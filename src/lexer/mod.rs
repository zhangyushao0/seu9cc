pub mod token;

use std::{cell::RefCell, iter::Peekable, str::Chars};

pub use token::{Span, Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    file_name: &'a str,
    file_content: &'a str,
    input_chars: Peekable<Chars<'a>>,
    start: usize,
    pub tokens: Vec<Token<'a>>,
}

impl Lexer<'_> {
    pub fn new<'a>(input: &'a str, file_name: &'a str, file_content: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            file_name,
            file_content,
            input_chars: input.chars().peekable(),
            start: 0,
            tokens: Vec::new(),
        }
    }
    fn input_chars_next(&mut self) -> Option<char> {
        let c = self.input_chars.next();
        if c.is_some() {
            self.start += 1;
        }
        c
    }
    fn input_chars_peek(&mut self) -> Option<char> {
        self.input_chars.peek().cloned()
    }

    pub fn scan(&mut self) {
        while let Some(c) = self.input_chars_peek() {
            // New line
            if c == '\n' {
                // to do! add preprocess strip newline and add str join
                // self.push_token(TokenKind::Symbol('\n'), 1);
                self.input_chars_next();
                continue;
            }

            // Whitespace
            if c.is_whitespace() {
                self.input_chars_next();
                continue;
            }

            // Line comment
            if self.starts_with("//") {
                while let Some(c) = self.input_chars_peek() {
                    if c == '\n' {
                        break;
                    }
                    self.input_chars_next();
                }
                continue;
            }

            // Block comment
            if self.starts_with("/*") {
                while let Some(c) = self.input_chars_next() {
                    if c == '*' && self.starts_with("*/") {
                        self.input_chars_next();
                        self.input_chars_next();
                        break;
                    }
                }
                continue;
            }

            // Character literal
            if c == '\'' {
                self.input_chars_next();
                let c = self.input_chars_next().unwrap();
                if self.input_chars_next() != Some('\'') {
                    panic!("Unterminated character literal");
                }
                self.push_token(TokenKind::Number(c as i64), 3);
                continue;
            }

            // String literal
            if c == '"' {
                self.input_chars_next();
                let mut s = String::new();
                while let Some(c) = self.input_chars_next() {
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                }
                self.push_token(TokenKind::Str(s.clone()), s.len() + 2);
                continue;
            }

            // Multi-letter symbol
            let next_two_chars = self.peek_n_chars(2);
            if let Some(tk_kind) = Token::from_str(&next_two_chars) {
                self.push_token(tk_kind, 2);
                continue;
            }

            // Single-letter symbol
            if let Some(tk_kind) = Token::from_char(c) {
                self.push_token(tk_kind, 1);
                self.input_chars_next();
                continue;
            }

            // Identifier or keyword

            if c.is_alphabetic() || c == '_' {
                let mut s = String::new();
                // s.push(c);
                while let Some(c) = self.input_chars_peek() {
                    if c.is_alphanumeric() || c == '_' {
                        s.push(c);
                        self.input_chars_next();
                    } else {
                        break;
                    }
                }
                let kind = match s.as_str() {
                    "int" => TokenKind::Int,
                    "return" => TokenKind::Return,
                    "extern" => TokenKind::Extern,
                    "typedef" => TokenKind::Typedef,
                    "char" => TokenKind::Char,
                    "void" => TokenKind::Void,
                    "struct" => TokenKind::Struct,
                    "_Bool" => TokenKind::Bool,
                    "if" => TokenKind::If,
                    "else" => TokenKind::Else,
                    "for" => TokenKind::For,
                    "do" => TokenKind::Do,
                    "while" => TokenKind::While,
                    "switch" => TokenKind::Switch,
                    "case" => TokenKind::Case,
                    "break" => TokenKind::Break,
                    "continue" => TokenKind::Continue,
                    "sizeof" => TokenKind::Sizeof,
                    "_Alignof" => TokenKind::Alignof,
                    "typeof" => TokenKind::Typeof,
                    _ => TokenKind::Ident(s.clone()),
                };
                self.push_token(kind, s.len());
                continue;
            }

            // Number literal
            if c.is_numeric() {
                self.scan_number();
                continue;
            }

            panic!(
                "cannot tokenize: {}, pos: {}, file: {}",
                c, self.start, self.file_name
            );
        }
    }

    // to do for octal, hex, binary
    fn scan_number(&mut self) {
        let mut s = String::new();
        while let Some(c) = self.input_chars_peek() {
            if c.is_numeric() {
                s.push(c);
                self.input_chars_next();
            } else {
                break;
            }
        }
        let n = s.parse().unwrap();
        self.push_token(TokenKind::Number(n), s.len());
    }

    fn push_token(&mut self, kind: TokenKind, len: usize) {
        let end = self.start + len;
        let span = Span {
            file_name: self.file_name,
            file_content: self.file_content,
            start: self.start,
            end,
        };
        self.start = end;
        self.tokens.push(Token { kind, span });
    }

    fn starts_with(&self, s: &str) -> bool {
        self.peek_n_chars(s.len()) == s
    }

    fn peek_n_chars(&self, n: usize) -> String {
        let mut result = String::new();
        let mut clone_iter = self.input_chars.clone();
        for _ in 0..n {
            if let Some(&c) = clone_iter.peek() {
                result.push(c);
                clone_iter.next();
            } else {
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = "int main(){ return 0;}";
        let file_name = "test.c";
        let file_content = input;
        let mut lexer = Lexer::new(input, file_name, file_content);
        lexer.scan();
        for token in lexer.tokens {
            println!("token kind: {:?}", token.kind);
        }
    }
}
