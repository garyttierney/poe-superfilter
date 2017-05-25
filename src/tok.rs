use regex::Regex;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt;

/// All tokens that can occur in superfilter syntax 
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    StrLiteral(String),
    Constant(String),
    VarIdentifier(String),
    Num(i64),
    Float(f64),
    Comment(String),
    BlockComment(String),
    LParen,
    RParen,
    Minus,
    Plus,
    Times,
    Div,
    Comma,
    NewLine,
    Gte,
    Gt,
    Lte,
    Lt,
    Eql,
    Show,
    Hide,
    Mixin,
    Import,
    If,
    True,
    False
}

impl Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Location {
    pub line: usize,
    pub pos: usize
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.pos)
    }
}

impl Default for Location {
    fn default() -> Location {
        Location { line: 1, pos: 0 }
    }
}

impl Location {
    fn add(&self, lines: usize, chars: usize) -> Location {
        Location {
            line: self.line.saturating_add(lines),
            pos: self.pos.saturating_add(chars)
        }
    }
}

/// Split a string into tokens, returning them as (start, token, end)
pub fn tokenize(s: &str) -> Vec<(Location, Tok, Location)> {
    let mut tokenizer = Tokenizer::new(s.chars());
    tokenizer.tokenize();
    tokenizer.tokens
}

struct Tokenizer<C: Iterator<Item=char>> {
    cursor: Location,
    chars: C,
    lookahead: Option<char>,
    tokens: Vec<(Location, Tok, Location)>,
    token_in_current_line: bool
}

impl<C: Iterator<Item=char>> Tokenizer<C> {
    fn new(chars: C) -> Tokenizer<C> {
        Tokenizer {
            cursor: Location { line: 1, pos: 0 },
            token_in_current_line: false,
            tokens: vec![],
            lookahead: None,
            chars: chars
        }
    }

    pub fn tokenize(&mut self) {
        self.next_char();
        while let Some(c) = self.lookahead {
            if !c.is_whitespace() || c == '\n' {
                self.read_token();
            } else {
                self.next_char();
            }
        }

        // insert newline at the end if none is present
        let last = self.tokens.last().cloned();
        match last {
            Some((_, Tok::NewLine, _)) => {}
            _ => self.push(Tok::NewLine, 1)
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.lookahead = self.chars.next();
        self.advance_cursor(1);
        self.lookahead
    }

    fn advance_cursor(&mut self, chars: usize) {
        self.cursor.pos += chars;
    }

    fn new_line(&mut self) {
        if self.token_in_current_line {
            self.push(Tok::NewLine, 1);
        }
        self.cursor.line += 1;
        self.cursor.pos = 0;
        self.token_in_current_line = false;
        self.next_char();
    }

    fn push(&mut self, token: Tok, length: usize) {
        self.token_in_current_line = true;
        self.tokens.push((self.cursor, token, self.cursor.add(0, length)));
    }

    fn next_and_push(&mut self, token: Tok, length: usize) {
        self.token_in_current_line = true;
        self.next_char();
        self.push(token, length);
    }

    fn read_token(&mut self) {
        if let Some(c) = self.lookahead {
            match c {
                '\n' => self.new_line(),
                '>' => {
                    let next = self.next_char();
                    match next {
                        Some('=') => self.next_and_push(Tok::Gte, 2),
                        _ => self.push(Tok::Gt, 1)
                    }
                }
                '<' => {
                    match self.next_char() {
                        Some('=') => self.next_and_push(Tok::Lte, 2),
                        _ => self.push(Tok::Lt, 1)
                    }
                }
                '=' => self.next_and_push(Tok::Eql, 1),
                '(' => self.next_and_push(Tok::LParen, 1),
                ')' => self.next_and_push(Tok::RParen, 1),
                '-' => self.next_and_push(Tok::Minus, 1),
                '+' => self.next_and_push(Tok::Plus, 1),
                '*' => self.next_and_push(Tok::Times, 1),
                ',' => self.next_and_push(Tok::Comma, 1),
                '/' => self.next_and_push(Tok::Div, 1),
                '"' => {
                    let tmp = self.take_quoted_string(c);
                    let length = tmp.len();
                    self.push(Tok::StrLiteral(tmp), length);
                }
                '$' => {
                    self.next_char();
                    if let Some(tmp) = self.take_identifier() {
                        let length = tmp.len();
                        self.push(Tok::VarIdentifier(tmp), length);
                    }
                }
                '#' => {
                    match self.next_char() {
                        Some('!') => {
                            let comment = self.take_while(None, |c| c != '\n');
                            let length = comment.len();
                            self.push(Tok::BlockComment(comment), length);
                        },
                        Some(any) => {
                            let comment = self.take_while(Some(any), |c| c != '\n');
                            let length = comment.len();
                            self.push(Tok::Comment(comment), length);
                        },
                        None => ()
                    };

                    self.new_line();
                },
                _ if c.is_alphabetic() => {
                    if let Some(tmp) = self.take_identifier() {
                        let length = tmp.len();
                        let tok = Tokenizer::<C>::match_keyword(tmp);
                        self.push(tok, length);
                    }
                    return;
                }
                _ if c.is_digit(10) => {
                    let tmp = self.take_while(Some(c), |c| c.is_digit(10) || c == '.');
                    if tmp.contains('.') {
                        self.push(Tok::Float(f64::from_str(&tmp).unwrap()), tmp.len());
                    } else {
                        self.push(Tok::Num(i64::from_str(&tmp).unwrap()), tmp.len());
                    }
                    return;
                }
                _ => {
                    panic!("invalid character: {:?}", c);
                }
            }
        }
    }

    /// Matches for keywords in unquoted strings
    fn match_keyword(id: String) -> Tok {
        match id.as_ref() {
            "Show" => Tok::Show,
            "Hide" => Tok::Hide,
            "Mixin" => Tok::Mixin,
            "Import" => Tok::Import,
            "if" => Tok::If,
            "True" => Tok::True,
            "False" => Tok::False,
            _ => Tok::Constant(id)
        }
    }

    fn take_identifier(&mut self) -> Option<String> {
        lazy_static! {
            static ref IDENT_CHAR_RX : Regex = Regex::new("[A-Za-z0-9_]").unwrap();
        }
        if let Some(c0) = self.lookahead {
            if !IDENT_CHAR_RX.is_match(&c0.to_string()) {
                return None;
            }
            return Some(self.take_while(Some(c0), |c| IDENT_CHAR_RX.is_match(&c.to_string())));
        }
        None
    }

    /// Consumes a quoted string (obeying escape sequences) and returns its contents.
    /// The return value does not contain the quotes themselves.
    fn take_quoted_string(&mut self, quote: char) -> String {
        let mut buf = String::new();

        while let Some(c) = self.next_char() {
            if c == quote {
                self.next_char();
                return buf;
            }

            if c == '\\' {
                let escaped_char = self.next_char();
                buf.push(escaped_char.unwrap());
            } else {
                buf.push(c);
            }
        }

        buf
    }

    /// Consumes characters, calling a closure each time until the closure returns false.
    /// Returns a String with all characters consumed up to that point.
    /// Taken from https://github.com/nikomatsakis/lalrpop/blob/master/lalrpop-test/src/util/tok.rs
    fn take_while<F>(&mut self, c0: Option<char>, f: F) -> String
        where F: Fn(char) -> bool
    {
        let mut buf = String::new();
        if let Some(c) = c0 { buf.push(c) }

        while let Some(c) = self.next_char() {
            if !f(c) {
                return buf;
            }

            buf.push(c);
        }

        buf
    }
}
