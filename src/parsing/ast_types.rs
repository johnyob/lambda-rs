use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Constant {
  Int(i64),
  Float(f64),
  Unit
}

pub use Constant::*;

impl Display for Constant {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      Int(n) => write!(f, "{}", n),
      Float(n) => write!(f, "{}", n),
      Unit => write!(f, "()")
    }
  }
}


#[derive(Debug, PartialEq, Eq)]
pub enum Primitive {
  Add,
  Sub,
  Div,
  Mul,
  Eq
}


pub use Primitive::*;

impl Display for Primitive {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match *self {
      Add => write!(f, "(+)"),
      Sub => write!(f, "(-)"),
      Div => write!(f, "(/)"),
      Mul => write!(f, "(*)"),
      Eq => write!(f, "(=)")
    }
  }
}
