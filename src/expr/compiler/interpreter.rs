// use crate::token::literal::Literal;
use super::{Expr, Environment, DType, Object};
use crate::token::TokenType;

pub trait Interpret {
    fn interpret(self, env: &mut Environment) -> Vec<u8>;
    fn interpret_new_env(self) -> Vec<u8>;
}

impl Interpret for Expr {
    fn interpret(self, env: &mut Environment) -> Vec<u8> {
        match self {
            Expr::Unary(_, _) => todo!(),
            Expr::Binary(left, op, right) => match op.ttype {
                TokenType::Equal => {
                    let name  = match *left {
                        Expr::BinaryOpt(left, op, _) => match op.ttype {
                            TokenType::Colon => match *left {
                                Expr::MsgEmission(None, name) => name.lexeme,
                                _ => panic!("expected identifier")
                            },
                            _ => panic!("expected declaration")
                        },
                        _ => panic!("expected declaration")
                    };
                    let val = right.interpret(env);
                    // env.define(name, val.clone(), Object { dtype: DType { size: 0, msgs: HashMap::new() }, address: 0 });
                    env.define(name, DType { size: 0, msgs: vec![] }, Object { dtype: DType { size: 0, msgs: vec![] }, address: 0 });
                    val
                },
                _ => panic!("unexpected binary operator")
            },
            Expr::MsgEmission(_, _) => todo!(),
            Expr::BinaryOpt(_, _, _) => todo!(),
            Expr::Object(exprs) => {
                let mut vals = vec![];
                for expr in exprs {
                    vals.append(&mut expr.interpret(env));
                }
                vals
            },
            Expr::Call(_, _) => todo!(),
            Expr::CodeBlock(_, exprs) => {
                let mut last = vec![];
                for expr in exprs {
                    last = expr.interpret(env);
                }
                last
            },
            // Expr::Identifier(name) => env.get(name.lexeme),
            Expr::Literal(_inner) => todo!(),
        }
    }

    fn interpret_new_env(self) -> Vec<u8> {
        self.interpret(&mut Environment::new())
    }
}