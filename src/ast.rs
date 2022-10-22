use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Term {
    Constant(i32),
    BinOp(Op, Box<Term>, Box<Term>),
    Var(u8),
    Abs(u8 /*var*/, Box<Term>),
    AbsP(Vec<u8>, Box<Term>), //TODO: AbsP into Abs should be implemented // TODO: for now only capital letters are supported for the "." operator
    App(Box<Term> /*func*/, Box<Term> /*arg*/),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(value) => write!(f, "{}", value),
            Term::Var(var) => write!(f, "{}", *var as char),
            Term::Abs(var, term) => write!(f, "(λ{} {})", *var as char, term),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::BinOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
            Term::AbsP(vars, term) => { //TODO: AbsP into Abs should be implemented
                let mut res = String::from("");
                for (i, var) in vars.iter().enumerate() {
                    res.push_str(format!("(λ{} ", *var as char).as_str());
                    if i == vars.len() - 1 {
                        res.push_str(format!("{})", term).as_str());
                    }
                }
                for _ in 0..vars.len() {
                    res.push_str(")");
                }
                write!(f, "{}", res)
            }
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Add => write!(f, "{}", '+'),
            Op::Sub => write!(f, "{}", '-'),
            Op::Mul => write!(f, "{}", '*'),
            Op::Div => write!(f, "{}", '/'),
        }
    }
}

impl Term {
    // Decide if `var` is free in `self`.
    fn is_free(&self, var: u8) -> bool {
        match self {
            Term::Var(var2) => var == *var2,
            Term::Abs(arg, body) => (var != *arg) && body.is_free(var),
            Term::App(t1, t2) => t1.is_free(var) || t2.is_free(var),
            Term::Constant(_) => true,
            _ => false,
        }
    }

    fn replace<'a>(&'a mut self, var: u8, subs: &Term) -> bool {
        match self {
            Term::Var(var2) => {
                if var == *var2 {
                    *self = subs.clone();
                }
                true
            }
            Term::Abs(arg, body) => {
                if var == *arg {
                    true
                } else if subs.is_free(*arg) {
                    false
                } else {
                    body.replace(var, subs)
                }
            }
            Term::App(t1, t2) => t1.replace(var, subs) && t2.replace(var, subs),
            Term::Constant(_) => true,
            _ => false,
        }
    }

    // Reduce `self` if possible.
    pub fn reduce(&mut self) {
        match self {
            // beta-reduction
            Term::App(t1, t2) => match &mut **t1 {
                Term::Abs(var, body) => {
                    if body.replace(*var, t2) {
                        *self = *body.clone();
                    }
                }
                _ => {
                    t1.reduce();
                    t2.reduce();
                }
            },
            _ => (),
        }
    }

    fn eval<'a>(&'a self, values: &mut std::collections::HashMap<u8, &'a Term>) -> i32 {
        match self {
            Term::Constant(value) => *value,
            Term::Var(var) => values[&var].eval(values),
            Term::BinOp(op, t1, t2) => match op {
                Op::Add => t1.eval(values) + t2.eval(values),
                Op::Sub => t1.eval(values) - t2.eval(values),
                Op::Mul => t1.eval(values) * t2.eval(values),
                Op::Div => t1.eval(values) / t2.eval(values),
            },
            Term::App(t1, t2) => match &**t1 {
                Term::Abs(arg, body) => {
                    values.insert(*arg, t2);
                    body.eval(values)
                }
                //TODO: Term::App (is it possible to evaluate App(App,..)?
                //Err
                _ => panic!(),
            },
            //Err
            _ => panic!(),
        }
    }

    pub fn evaluate(&self) -> i32 {
        self.eval(&mut std::collections::HashMap::new())
    }
}
