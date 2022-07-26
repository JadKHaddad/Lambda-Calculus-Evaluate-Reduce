use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub enum Term {
    Constant(i32),
    BinOp(Op, Box<Term>, Box<Term>),
    Var(u8),
    Abs(u8, Box<Term>),
    App(Box<Term> /*func*/, Box<Term> /*arg*/),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(value) => write!(f, "{}", value),
            Term::Var(var) => write!(f, "{}", *var as char),
            Term::Abs(var, term) => write!(f, "(λ{}. {})", *var as char, term),
            Term::App(t1, t2) => write!(f, "{}({})", t1, t2),
            Term::BinOp(op, t1, t2) => write!(f, "{} {} {}", t1, op, t2),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    Plus,
    Minus,
    Times,
    Divide,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Plus => write!(f, "{}", '+'),
            Op::Minus => write!(f, "{}", '-'),
            Op::Times => write!(f, "{}", '*'),
            Op::Divide => write!(f, "{}", '/'),
        }
    }
}
