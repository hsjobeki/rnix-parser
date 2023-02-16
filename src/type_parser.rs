//! The parser: turns a series of tokens into an AST

use std::collections::VecDeque;

use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, TextRange, TextSize};

use crate::{
    parser::ParseError,
    tokenizer::Token,
    NixLanguage,
    SyntaxKind::{self, *},
    TokenSet,
};

struct TypeParser<'a, I>
where
    I: Iterator<Item = Token<'a>>,
{
    // GreenNodeBuilder from 'rowan' crate
    builder: &'a mut GreenNodeBuilder<'a>,
    // List of ParseErrors
    errors: Vec<ParseError>,

    //List of Tokens (trivia)
    trivia_buffer: Vec<Token<'a>>,
    // TwoSided Queue for Tokens
    buffer: VecDeque<Token<'a>>,
    // An Iterator over Tokens
    iter: I,
    // Tracks the amount of consumed Tokens
    consumed: TextSize,

    // Recursion depth, used for avoiding stack overflows. This may be incremented
    // by any method as long as it is decremented when that method returns.
    depth: u32,
}
impl<'a, I> TypeParser<'a, I>
where
    I: Iterator<Item = Token<'a>>,
{
    fn new(iter: I, builder: &'a mut GreenNodeBuilder<'a>) -> Self {
        Self {
            builder,
            errors: Vec::new(),

            trivia_buffer: Vec::with_capacity(1),
            buffer: VecDeque::with_capacity(1),
            iter,
            consumed: TextSize::from(0),

            depth: 0,
        }
    }

    fn get_text_position(&self) -> TextSize {
        self.consumed
    }

    fn peek_raw(&mut self) -> Option<&Token<'a>> {
        if self.buffer.is_empty() {
            if let Some(token) = self.iter.next() {
                self.buffer.push_back(token);
            }
        }
        self.buffer.front()
    }
    fn drain_trivia_buffer(&mut self) {
        for (t, s) in self.trivia_buffer.drain(..) {
            self.consumed += TextSize::of(s);
            self.builder.token(NixLanguage::kind_to_raw(t), s);
        }
    }
    fn eat_trivia(&mut self) {
        self.peek();
        self.drain_trivia_buffer();
    }
    fn start_node(&mut self, kind: SyntaxKind) {
        self.eat_trivia();
        self.builder.start_node(NixLanguage::kind_to_raw(kind));
    }
    fn checkpoint(&mut self) -> Checkpoint {
        self.eat_trivia();
        self.builder.checkpoint()
    }
    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, NixLanguage::kind_to_raw(kind));
    }
    fn finish_node(&mut self) {
        self.builder.finish_node();
    }
    fn start_error_node(&mut self) -> TextSize {
        self.start_node(NODE_ERROR);
        self.get_text_position()
    }
    fn finish_error_node(&mut self) -> TextSize {
        self.finish_node();
        self.get_text_position()
    }
    fn bump(&mut self) {
        match self.try_next() {
            Some((token, s)) => {
                if token.is_trivia() {
                    self.trivia_buffer.push((token, s))
                } else {
                    self.drain_trivia_buffer();
                    self.manual_bump(s, token);
                }
            }
            None => self.errors.push(ParseError::UnexpectedEOF),
        }
    }
    fn try_next(&mut self) -> Option<Token<'a>> {
        self.buffer.pop_front().or_else(|| self.iter.next())
    }
    fn manual_bump(&mut self, s: &str, token: SyntaxKind) {
        self.consumed += TextSize::of(s);
        self.builder.token(NixLanguage::kind_to_raw(token), s)
    }
    fn peek_data(&mut self) -> Option<&Token<'a>> {
        while self.peek_raw().map(|&(t, _)| t.is_trivia()).unwrap_or(false) {
            self.bump();
        }
        self.peek_raw()
    }
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.peek_data().map(|&(t, _)| t)
    }
    fn expect_peek_any(&mut self, allowed_slice: &[SyntaxKind]) -> Option<SyntaxKind> {
        let allowed = TokenSet::from_slice(allowed_slice);

        let next = match self.peek() {
            None => None,
            Some(kind) if allowed.contains(kind) => Some(kind),
            Some(kind) => {
                let start = self.start_error_node();
                loop {
                    self.bump();
                    if self.peek().map(|kind| allowed.contains(kind)).unwrap_or(true) {
                        break;
                    }
                }
                let end = self.finish_error_node();
                self.errors.push(ParseError::UnexpectedWanted(
                    kind,
                    TextRange::new(start, end),
                    allowed_slice.to_vec().into_boxed_slice(),
                ));

                self.peek()
            }
        };
        if next.is_none() {
            self.errors
                .push(ParseError::UnexpectedEOFWanted(allowed_slice.to_vec().into_boxed_slice()));
        }
        next
    }
    fn expect(&mut self, expected: SyntaxKind) {
        if self.expect_peek_any(&[expected]).is_some() {
            self.bump();
        }
    }

    fn expect_ident(&mut self) {
        if self.expect_peek_any(&[TOKEN_IDENT]).is_some() {
            self.start_node(NODE_IDENT);
            self.bump();
            self.finish_node()
        }
    }

    fn parse_type() -> () {
        // type_recursion += 1;
        // match self.peek() {
        //     Some(TOKEN_SEMICOLON) => {
        //         if type_recursion > 0 {
        //             self.bump();
        //             self.eat_trivia();
        //             break;
        //         // semicolon must not be the first character
        //         } else {
        //             let start = self.start_error_node();
        //             loop {
        //                 match self.peek() {
        //                     Some(
        //                         TOKEN_MULTILINE_COMMENT_END
        //                         | TOKEN_EXAMPLE_COMMENT,
        //                     ) => break,
        //                     None => break,
        //                     _ => self.bump(),
        //                 };
        //             }
        //             let end = self.finish_error_node();
        //             self.errors.push(ParseError::Unexpected(TextRange::new(
        //                 start, end,
        //             )));
        //             break;
        //         }
        //     }
        //     Some(TOKEN_TYPE) => {
        //         // let checkpoint = self.checkpoint();
        //         // self.start_node_at(checkpoint, NODE_TYPE);
        //         self.bump();
        //         // self.finish_node();
        //     }
        //     Some(_) => {
        //         self.bump();
        //     }
        //     _ => {
        //         self.errors.push(ParseError::UnexpectedEOF);
        //         break;
        //     }
        // }
    }
    fn parse_string(&mut self) {
        self.start_node(NODE_STRING);
        self.expect(TOKEN_STRING_START);

        loop {
            match self.expect_peek_any(&[
                TOKEN_STRING_END,
                TOKEN_STRING_CONTENT,
                // TOKEN_INTERPOL_START,
            ]) {
                Some(TOKEN_STRING_CONTENT) => self.bump(),
                // Some(TOKEN_INTERPOL_START) => {
                //     self.start_node(NODE_INTERPOL);
                //     self.bump();
                //     self.parse_expr();
                //     self.expect(TOKEN_INTERPOL_END);
                //     self.finish_node();
                // }

                // handled by expect_peek_any
                _ => break,
            }
        }
        self.expect(TOKEN_STRING_END);

        self.finish_node();
    }
    //parse interpolation // not really needed since we can only have Ident Node inside
    fn parse_dynamic(&mut self) {
        self.start_node(NODE_DYNAMIC);
        self.bump();
        self.expect_ident();
        self.bump();
        self.finish_node();
    }

    fn parse_attr(&mut self) {
        match self.peek() {
            Some(TOKEN_INTERPOL_START) => self.parse_dynamic(),
            Some(TOKEN_STRING_START) => self.parse_string(),
            _ => {
                if self.expect_peek_any(&[TOKEN_IDENT, TOKEN_OR]).is_some() {
                    self.start_node(NODE_IDENT);
                    let (_, s) = self.try_next().unwrap();
                    self.manual_bump(s, TOKEN_IDENT);
                    self.finish_node()
                }
            }
        }
    }

    fn parse_attrpath(&mut self) {
        self.start_node(NODE_ATTRPATH);
        self.parse_attr();
        // loop {

        //     if self.peek() == Some(T![.]) {
        // self.bump();
        //     } else {
        //         break;
        //     }
        // }
        self.finish_node();
    }

    fn parse_set(&mut self, until: SyntaxKind) {
        loop {
            match self.peek() {
                None => break,
                token if token == Some(until) => break,
                Some(_) => {
                    self.start_node(NODE_ATTRPATH_VALUE);
                    self.parse_attrpath();
                    self.expect(T![::]);
                    self.parse_expr();
                    self.expect(T![;]);
                    self.finish_node();
                }
            }
        }
        self.bump(); // the final close, like '}'
    }

    pub fn parse_expr(&mut self) -> Checkpoint {
        // Limit chosen somewhat arbitrarily
        if self.depth >= 512 {
            self.errors.push(ParseError::RecursionLimitExceeded);
            // Consume tokens to the end of the file. Erroring without bumping might cause
            // infinite looping elsewhere.
            self.start_error_node();
            while self.peek().is_some() {
                self.bump()
            }
            self.finish_error_node();
            return self.checkpoint();
        }
        self.depth += 1;
        let out = match self.peek() {
            Some(T![let]) => {
                let checkpoint = self.checkpoint();
                self.bump();

                self.start_node_at(checkpoint, NODE_LET_IN);
                self.parse_set(T![in]);
                self.parse_expr();
                self.finish_node();

                checkpoint
            }
            t => {
                let checkpoint = self.checkpoint();
                println!("Yet unhandled {t:?}");
                checkpoint
            }
        };
        self.depth -= 1;
        out
    }
}

/// Parse tokens into an AST
pub fn parse<'s, I>(iter: I, builder: &'s mut GreenNodeBuilder<'s>) -> (GreenNode, Vec<ParseError>)
where
    I: Iterator<Item = Token<'s>>,
{
    let mut parser = TypeParser::new(iter, builder);
    parser.builder.start_node(NixLanguage::kind_to_raw(NODE_ROOT));
    parser.parse_expr();
    parser.eat_trivia();
    if parser.peek().is_some() {
        let start = parser.start_error_node();
        while parser.peek().is_some() {
            parser.bump();
        }
        let end = parser.finish_error_node();
        parser.errors.push(ParseError::UnexpectedExtra(TextRange::new(start, end)));
        parser.eat_trivia();
    }
    parser.builder.finish_node();
    (parser.builder.finish(), parser.errors)
}
