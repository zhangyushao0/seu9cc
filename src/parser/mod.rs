use crate::lexer;
struct parser<'a> {
    tokens: Vec<lexer::Token<'a>>,
}

impl parser<'_> {
    fn new<'a>(tokens: Vec<lexer::Token<'a>>) -> parser<'a> {
        parser { tokens }
    }
    fn parse(&mut self) {
        for token in self.tokens.iter() {
            println!("token kind: {:?}", token.kind);
        }
    }

    fn toplevel(&mut self) {
        // todo typedef and extern
    }
    fn decl_specifiers(&mut self) {
        // todo int, char, void, struct, bool
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
