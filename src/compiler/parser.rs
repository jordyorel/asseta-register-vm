// Add this import or definition at the top of the file
use crate::compiler::lexer::Token;

#[derive(Debug)]
pub enum ASTNode {
    VariableDeclaration {
        mutable: bool,
        name: String,
        value: Option<Box<ASTNode>>,
    },
    Assignment {
        target: String,
        value: Box<ASTNode>,
    },
    BinaryOp {
        op: String,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Identifier(String),
    Number(i32),
    ForLoop {
        variable: String,
        start: Box<ASTNode>,
        end: Box<ASTNode>,
        inclusive: bool,
        body: Vec<ASTNode>,
    },
    Print(Vec<ASTNode>),
    Block(Vec<ASTNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut statements = Vec::new();
        
        while !self.is_at_end() {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            } else {
                self.advance();
            }
        }
        
        statements
    }
    
    fn statement(&mut self) -> Option<ASTNode> {
        match self.peek() {
            Some(Token::Keyword(kw)) if kw == "mut" => self.variable_declaration(),
            Some(Token::Keyword(kw)) if kw == "for" => self.for_loop(),
            Some(Token::Keyword(kw)) if kw == "print" => self.print_statement(),
            Some(Token::Identifier(_)) => self.assignment(),
            _ => None,
        }
    }
    
    fn for_loop(&mut self) -> Option<ASTNode> {
        self.consume_keyword("for")?;
        
        let variable = self.consume_identifier()?;
        
        self.consume_keyword("in")?;
        
        let start = self.expression()?;
        
        let inclusive = match self.peek() {
            Some(Token::RangeInclusive) => {
                self.advance();
                true
            }
            Some(Token::RangeExclusive) => {
                self.advance();
                false
            }
            _ => return None,
        };
        
        let end = self.expression()?;
        self.consume(Token::Colon)?;
        self.consume(Token::Newline)?;
        
        // Handle indentation
        let mut body = Vec::new();
        if self.consume(Token::Indent).is_some() {
            while self.peek() != Some(&Token::Dedent) && !self.is_at_end() {
                if let Some(stmt) = self.statement() {
                    body.push(stmt);
                } else {
                    self.advance();
                }
            }
            self.consume(Token::Dedent);
        }
        
        Some(ASTNode::ForLoop {
            variable,
            start: Box::new(start),
            end: Box::new(end),
            inclusive,
            body,
        })
    }
    
    // Implement other parsing methods...

    fn advance(&mut self) -> Option<&Token> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn consume(&mut self, token: Token) -> Option<&Token> {
        if self.peek() == Some(&token) {
            self.advance()
        } else {
            None
        }
    }

    fn consume_keyword(&mut self, kw: &str) -> Option<()> {
        match self.peek() {
            Some(Token::Keyword(k)) if k == kw => {
                self.advance();
                Some(())
            }
            _ => None,
        }
    }

    fn consume_identifier(&mut self) -> Option<String> {
        match self.peek() {
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.advance();
                Some(name)
            }
            _ => None,
        }
    }

    // Variable declaration: "mut" <identifier> ["=" <expression>]
    fn variable_declaration(&mut self) -> Option<ASTNode> {
        self.consume_keyword("mut")?;
        let name = self.consume_identifier()?;
        
        let value = if self.consume(Token::Operator("=".to_string())).is_some() {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        
        Some(ASTNode::VariableDeclaration {
            mutable: true,
            name,
            value,
        })
    }

    // Assignment: <identifier> "=" <expression>
    fn assignment(&mut self) -> Option<ASTNode> {
        let target = self.consume_identifier()?;
        self.consume(Token::Operator("=".to_string()))?;
        let value = Box::new(self.expression()?);
        
        Some(ASTNode::Assignment { target, value })
    }

    // Print statement: "print" <expression>
    fn print_statement(&mut self) -> Option<ASTNode> {
        self.consume_keyword("print")?;
        let expr = self.expression()?;
        Some(ASTNode::Print(vec![expr]))
    }

    // Expression parsing with operator precedence
    fn expression(&mut self) -> Option<ASTNode> {
        self.term()
    }

    fn term(&mut self) -> Option<ASTNode> {
        let mut left = self.factor()?;
        
        while let Some(Token::Operator(op)) = self.peek() {
            if op == "+" || op == "-" {
                let op = op.clone();
                self.advance();
                let right = self.factor()?;
                left = ASTNode::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Some(left)
    }

    fn factor(&mut self) -> Option<ASTNode> {
        let mut left = self.primary()?;
        
        while let Some(Token::Operator(op)) = self.peek() {
            if op == "*" || op == "/" {
                let op = op.clone();
                self.advance();
                let right = self.primary()?;
                left = ASTNode::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        
        Some(left)
    }

    fn primary(&mut self) -> Option<ASTNode> {
        match self.advance() {
            Some(Token::Number(n)) => Some(ASTNode::Number(*n)),
            Some(Token::Identifier(name)) => Some(ASTNode::Identifier(name.clone())),
            _ => None,
        }
    }
}