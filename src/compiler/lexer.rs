#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Colon,
    RangeExclusive,
    RangeInclusive,
    Operator(String),
    Newline,
    Indent,
    Dedent,
    EOF,
}

pub struct Lexer {
    chars: Vec<char>,
    position: usize,
    indent_stack: Vec<usize>,
    pending_dedents: usize,
    at_line_start: bool,
    current_indent: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            chars: source.chars().collect(),
            position: 0,
            indent_stack: vec![0],
            pending_dedents: 0,
            at_line_start: true,
            current_indent: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        
        // Add remaining dedents at EOF
        for _ in 1..self.indent_stack.len() {
            tokens.push(Token::Dedent);
        }
        
        tokens.push(Token::EOF);
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        // Handle dedents first
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Some(Token::Dedent);
        }
        
        // Skip whitespace (but not newlines)
        self.skip_non_newline_whitespace();
        
        // Handle start of line indentation
        if self.at_line_start {
            self.at_line_start = false;
            let indent = self.skip_whitespace();
            
            if self.peek() == Some('\n') || self.peek().is_none() {
                // Blank line, reset and continue
                self.at_line_start = true;
                return self.next_token();
            }
            
            let current_indent = self.indent_stack.last().unwrap();
            
            if indent > *current_indent {
                self.indent_stack.push(indent);
                return Some(Token::Indent);
            } else if indent < *current_indent {
                while indent < *self.indent_stack.last().unwrap() {
                    self.indent_stack.pop();
                    self.pending_dedents += 1;
                }
                return Some(Token::Dedent);
            }
        }
        
        let c = match self.advance() {
            Some(c) => c,
            None => return None,
        };
        
        match c {
            '\n' => {
                self.at_line_start = true;
                self.current_indent = 0;
                Some(Token::Newline)
            }
            ':' => Some(Token::Colon),
            '.' => {
                if self.peek() == Some('.') {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        Some(Token::RangeInclusive)
                    } else {
                        Some(Token::RangeExclusive)
                    }
                } else {
                    Some(Token::Operator(".".to_string()))
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = c.to_string();
                while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = self.peek() {
                    ident.push(self.advance().unwrap());
                }
                match ident.as_str() {
                    "mut" | "for" | "in" | "print" => Some(Token::Keyword(ident)),
                    _ => Some(Token::Identifier(ident)),
                }
            }
            '0'..='9' => {
                let mut num = c.to_string();
                while let Some('0'..='9') = self.peek() {
                    num.push(self.advance().unwrap());
                }
                num.parse().ok().map(Token::Number)
            }
            '+' | '-' | '*' | '/' | '=' => Some(Token::Operator(c.to_string())),
            _ => None,
        }
    }
    
    // Moves the lexer forward by one character and returns it
    fn advance(&mut self) -> Option<char> {
        if self.position < self.chars.len() {
            let c = self.chars[self.position];
            self.position += 1;
            Some(c)
        } else {
            None
        }
    }

    // Peeks at the next character without consuming it
    fn peek(&self) -> Option<char> {
        if self.position < self.chars.len() {
            Some(self.chars[self.position])
        } else {
            None
        }
    }

    // Skips all whitespace characters except newlines
    fn skip_non_newline_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    // Skips whitespace (spaces/tabs) and returns the count of characters skipped
    fn skip_whitespace(&mut self) -> usize {
        let mut count = 0;
        while let Some(c) = self.peek() {
            if c == ' ' || c == '\t' {
                self.advance();
                count += 1;
            } else {
                break;
            }
        }
        count
    }
    
}