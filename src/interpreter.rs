// src/interpreter.rs

use crate::parser::{ASTNode};
use crate::lexer::{Token};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>) {
        for node in nodes {
            self.evaluate(node);
        }
    }

    fn evaluate(&mut self, node: ASTNode) -> f64 {
        match node {
            ASTNode::VariableDeclaration(name, expr) => {
                let value = self.evaluate(*expr);
                self.variables.insert(name, value);
                0.0
            }
            ASTNode::Variable(name) => *self.variables.get(&name).expect("Undefined variable"),
            ASTNode::Print(expr) => {
                print!("{}", self.evaluate(*expr));
                0.0
            }
            ASTNode::PrintLn(expr) => {
                println!("{}", self.evaluate(*expr));
                0.0
            }
            ASTNode::Number(n) => n,
            ASTNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                match operator {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Multiply => left_val * right_val,
                    Token::Divide => left_val / right_val,
                    _ => panic!("Unexpected operator: {:?}", operator),
                }
            }
        }
    }
}
