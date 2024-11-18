use crate::common::*;
use crate::lexer::{self, Token};
use std::iter::Peekable;
use std::vec::IntoIter;
struct parser<'a> {
    input_tokens: Peekable<IntoIter<lexer::Token<'a>>>,
}

impl parser<'_> {
    fn new<'a>(tokens: Vec<lexer::Token<'a>>) -> parser<'a> {
        parser {
            input_tokens: tokens.into_iter().peekable(),
        }
    }
    fn parse(&mut self) {
        self.toplevel();
    }

    fn input_tokens_next(&mut self) -> Option<lexer::Token> {
        self.input_tokens.next()
    }
    fn input_tokens_consume(&mut self, kind: lexer::TokenKind) -> bool {
        if let Some(token) = self.input_tokens.peek() {
            if token.kind == kind {
                self.input_tokens.next();
                return true;
            }
        }
        false
    }
    fn toplevel(&mut self) {
        // to do typedef extern
        let mut token = self.decl_specifiers();

        while self.input_tokens_consume(lexer::TokenKind::Symbol('*')) {
            token = Type::new_ptr(token);
        }

        let ident = self.ident();

        // function
        if self.input_tokens_consume(lexer::TokenKind::Symbol('(')) {
            // to do
            // let params = self.params();
            // let body = self.compound_stmt();
        }
    }
    fn decl_specifiers(&mut self) -> Type {
        let mut token = self.input_tokens_next().unwrap();

        match token.kind {
            lexer::TokenKind::Void => Type::new_void(),
            lexer::TokenKind::Bool => Type::new_bool(),
            lexer::TokenKind::Char => Type::new_char(),
            lexer::TokenKind::Int => Type::new_int(),
            lexer::TokenKind::Struct => {
                panic!("Struct is not implemented yet");
            }
            _ => panic!("Invalid type"),
        }
    }
    fn ident(&mut self) -> String {
        let token = self.input_tokens_next().unwrap();
        match token.kind {
            lexer::TokenKind::Ident(s) => s,
            _ => panic!("Invalid identifier"),
        }
    }
    fn params(&mut self) {
        // to do
    }
    fn params_decl(&mut self) -> Var {
        let ty = self.decl_specifiers();
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser() {
        let input = "int main(){ return 0;}";
        let file_name = "test.c";
        let file_content = input;
        let mut lexer = lexer::Lexer::new(input, file_name, file_content);
        lexer.scan();
        let tokens = lexer.tokens;
        let mut parser = parser::new(tokens);
        parser.parse();
    }
}
