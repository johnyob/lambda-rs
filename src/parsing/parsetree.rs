use crate::parsing::ast_types::*;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Expression {
  Const(Constant),
  Prim(Primitive),
  Var(String),
  Fun(String, BoxExpression),
  App(BoxExpression, BoxExpression),
  IfThenElse(BoxExpression, BoxExpression, BoxExpression)
}

pub use Expression::*;

impl Display for Expression {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Const(const_) => write!(f, "{}", const_),      
      Prim(prim) => write!(f, "%prim {}", prim),
      Var(var) => write!(f, "{}", var),
      Fun(var, exp) => write!(f, "fun {} -> {}", var, exp),
      App(exp1, exp2) => write!(f, "({}) {}", exp1, exp2),
      IfThenElse(exp1, exp2, exp3) => write!(f, "if {} then {} else {}", exp1, exp2, exp3)
    }
  }
}

pub type BoxExpression = Box<Expression>;




