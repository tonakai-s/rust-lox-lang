use std::{collections::HashMap, fmt, iter::Peekable, vec::IntoIter};

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Start,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literal
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}
#[derive(Debug)]
#[allow(dead_code)]
enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    token_type: TokenType,
    literal: Option<Literal>,
    line: usize,
}
impl Token {
    fn new(token_type: TokenType, literal: Option<Literal>, line: usize) -> Self {
        Self {
            token_type,
            literal,
            line,
        }
    }
}

struct Scanner {
    column: usize,
    line: usize,
    source: Peekable<IntoIter<char>>,
}
#[allow(dead_code)]
impl Scanner {
    fn new(source: String) -> Self {
        Self {
            column: 1,
            line: 1,
            source: source.chars().collect::<Vec<char>>().into_iter().peekable(),
        }
    }
    fn advance_match(&mut self, ch: char) -> bool {
        self.source.next();

        if let Some(ch_next) = self.source.peek() {
            if *ch_next == ch {
                self.column += 1;
                self.source.next();
                return true;
            }
        }
        false
    }
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }
    fn next(&mut self) -> Option<char> {
        self.column += 1;
        self.source.next()
    }
    fn next_line(&mut self) -> Option<char> {
        self.column = 0;
        self.line += 1;
        self.source.next()
    }
    fn skip_line(&mut self) {
        while let Some(ch) = self.next() {
            if ch == '\n' {
                self.column = 0;
                self.line += 1;
            }
        }
    }
    fn string(&mut self) -> String {
        self.next();

        let mut buff = String::new();
        while let Some(ch) = self.peek() {
            if *ch == '\n' {
                self.line += 1;
                continue;
            };
            if *ch == '\0' {
                todo!("Implement error: Unterminated string.")
            }
            if *ch == '"' {
                break;
            };
            buff.push(self.next().unwrap());
        }

        buff
    }
    fn number(&mut self) -> f64 {
        let mut buff = String::new();
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            buff.push(self.next().unwrap());
        }

        match buff.parse::<f64>() {
            std::result::Result::Ok(num) => num,
            Err(_) => todo!("Implement: Parse num error."),
        }
    }
    fn identifier(&mut self) -> String {
        let mut buff = String::new();
        while let Some(ch) = self.peek() {
            if !ch.is_alphanumeric() {
                break;
            }
            buff.push(self.next().unwrap());
        }

        buff
    }
}

#[derive(Debug)]
pub struct ParserError {
    _file: String,
    error: String,
    line: usize,
    column: usize,
}
impl ParserError {
    fn new(file: String, error: String, line: usize, column: usize) -> Self {
        Self {
            _file: file,
            error,
            line,
            column,
        }
    }
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {} at {}:{}", self.error, self.line, self.column)
    }
}

pub struct Parser {
    keywords: HashMap<&'static str, TokenType>,
    scanner: Scanner,
    pub tokens: Vec<Token>,
    pub errors: Vec<ParserError>,
}
impl Parser {
    pub fn new(source: String) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);

        Self {
            keywords,
            scanner: Scanner::new(source),
            tokens: vec![],
            errors: vec![],
        }
    }

    pub fn report(&self) {
        for error in &self.errors {
            println!("{}", error);
        }
    }

    pub fn scan_tokens(&mut self) {
        while let Some(ch) = self.scanner.peek() {
            match ch {
                '(' => self.add_token_advance(TokenType::LeftBrace, None),
                ')' => self.add_token_advance(TokenType::RightBrace, None),
                '{' => self.add_token_advance(TokenType::RightParen, None),
                '}' => self.add_token_advance(TokenType::LeftParen, None),
                ',' => self.add_token_advance(TokenType::Comma, None),
                '.' => self.add_token_advance(TokenType::Dot, None),
                '-' => self.add_token_advance(TokenType::Minus, None),
                '+' => self.add_token_advance(TokenType::Plus, None),
                ';' => self.add_token_advance(TokenType::Semicolon, None),
                '*' => self.add_token_advance(TokenType::Start, None),
                '!' => {
                    let tkn_type = if self.scanner.advance_match('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(tkn_type, None);
                }
                '=' => {
                    let tkn_type = if self.scanner.advance_match('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(tkn_type, None);
                }
                '<' => {
                    let tkn_type = if self.scanner.advance_match('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(tkn_type, None);
                }
                '>' => {
                    let tkn_type = if self.scanner.advance_match('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(tkn_type, None);
                }
                '/' => {
                    if self.scanner.advance_match('/') {
                        self.scanner.skip_line();
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }
                ' ' | '\r' | '\t' => {
                    self.scanner.next();
                }
                '\n' => {
                    self.scanner.next_line();
                }
                '"' => {
                    let str = self.scanner.string();
                    self.add_token_advance(TokenType::String, Some(Literal::String(str)));
                }
                _ => {
                    if ch.is_ascii_digit() {
                        let num = self.scanner.number();
                        self.add_token(TokenType::Number, Some(Literal::Number(num)));
                    } else if ch.is_alphabetic() || *ch == '_' {
                        let identifier = self.scanner.identifier();
                        if let Some(tkn_type) = self.keywords.get(identifier.as_str()) {
                            self.add_token(tkn_type.clone(), None);
                        } else {
                            self.add_token(
                                TokenType::Identifier,
                                Some(Literal::Identifier(identifier)),
                            );
                        }
                    } else {
                        self.add_error("Unexpected character".to_string());
                        // todo!("Implement: Error report");
                    }
                }
            }
        }
    }

    // TODO: Make a macro! (???)
    fn add_token(&mut self, tkn_type: TokenType, literal: Option<Literal>) {
        self.tokens
            .push(Token::new(tkn_type, literal, self.scanner.line));
    }
    fn add_token_advance(&mut self, tkn_type: TokenType, literal: Option<Literal>) {
        self.tokens
            .push(Token::new(tkn_type, literal, self.scanner.line));
        self.scanner.next();
    }
    fn add_error(&mut self, error: String) {
        self.scanner.next();
        let error = ParserError::new("".into(), error, self.scanner.line, self.scanner.column);
        self.errors.push(error);
    }
}
