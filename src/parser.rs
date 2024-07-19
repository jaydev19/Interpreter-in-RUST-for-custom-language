// src/parser.rs

use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum ASTNode {
    VariableDeclaration(String, Box<ASTNode>),
    Variable(String),
    Print(Box<ASTNode>),
    PrintLn(Box<ASTNode>),
    Number(f64),
    BinaryOp {
        left: Box<ASTNode>,
        operator: Token,
        right: Box<ASTNode>,
    },
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.current_token = parser.lexer.next_token();
        parser
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            let node = self.parse_statement();
            nodes.push(node);
        }
        nodes
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token {
            Token::Let => {
                self.next_token();
                if let Token::Identifier(name) = self.current_token.clone() {
                    self.next_token();
                    self.expect_token(Token::Assign);
                    let expr = self.parse_expression();
                    self.expect_token(Token::Semicolon);
                    ASTNode::VariableDeclaration(name, Box::new(expr))
                } else {
                    panic!("Expected identifier after 'let'");
                }
            }
            Token::Print => {
                self.next_token();
                let expr = self.parse_expression();
                self.expect_token(Token::Semicolon);
                ASTNode::Print(Box::new(expr))
            }
            Token::PrintLn => {
                self.next_token();
                let expr = self.parse_expression();
                self.expect_token(Token::Semicolon);
                ASTNode::PrintLn(Box::new(expr))
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        if let Token::Identifier(name) = self.current_token.clone() {
            self.next_token();
            ASTNode::Variable(name)
        } else {
            self.parse_term()
        }
    }

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let operator = self.current_token.clone();
            self.next_token();
            let right = self.parse_factor();
            node = ASTNode::BinaryOp {
                left: Box::new(node),
                operator,
                right: Box::new(right),
            };
        }
        node
    }

    fn parse_factor(&mut self) -> ASTNode {
        match self.current_token {
            Token::Number(n) => {
                self.next_token();
                ASTNode::Number(n)
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_expression_statement(&mut self) -> ASTNode {
        let node = self.parse_expression();
        self.expect_token(Token::Semicolon);
        node
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect_token(&mut self, token: Token) {
        if self.current_token != token {
            panic!("Expected token: {:?}, found: {:?}", token, self.current_token);
        }
        self.next_token();
    }
}
