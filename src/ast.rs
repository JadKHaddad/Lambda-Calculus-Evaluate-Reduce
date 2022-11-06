use std::collections::HashSet;
use std::convert;
use std::error::Error as StdError;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Term {
    Constant(i32),
    BinOp(Op, Box<Term>, Box<Term>),
    Var(u8),
    Abs(u8 /*arg*/, Box<Term>),
    App(Box<Term> /*func*/, Box<Term> /*arg*/),
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}
#[derive(Debug, Clone)]
pub enum Error {
    NewVariableNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NewVariableNotFound => write!(f, "Cannot find new variable"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            Error::NewVariableNotFound => "Cannot find new variable",
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Constant(value) => write!(f, "{}", value),
            Term::Var(var) => write!(f, "{}", *var as char),
            Term::Abs(var, term) => write!(f, "(λ{} {})", *var as char, term),
            Term::App(t1, t2) => write!(f, "({} {})", t1, t2),
            Term::BinOp(op, t1, t2) => write!(f, "({} {} {})", t1, op, t2),
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
            Op::Eq => write!(f, "{}", '='),
        }
    }
}

impl Term {
    pub fn cond_0() -> Self {
        Term::Abs(b'x', Box::new(Term::Abs(b'y', Box::new(Term::Var(b'x')))))
    }

    pub fn cond_1() -> Self {
        Term::Abs(b'x', Box::new(Term::Abs(b'y', Box::new(Term::Var(b'y')))))
    }

    pub fn get_vars(&self) -> HashSet<u8> {
        match self {
            Term::Constant(_) => HashSet::new(),
            Term::Var(var) => {
                let mut vars = HashSet::new();
                vars.insert(*var);
                vars
            }
            Term::Abs(var, term) => {
                let mut vars = term.get_vars();
                vars.remove(var);
                vars
            }
            Term::App(t1, t2) => {
                let mut vars = t1.get_vars();
                vars.extend(t2.get_vars());
                vars
            }
            Term::BinOp(_, t1, t2) => {
                let mut vars = t1.get_vars();
                vars.extend(t2.get_vars());
                vars
            }
        }
    }

    pub fn get_bound_vars(&self) -> HashSet<Term> {
        match self {
            Term::Abs(arg, body) => {
                let mut x: HashSet<Term> = HashSet::new();
                x.insert(Term::Var(*arg));
                let y = body.get_bound_vars();
                x.extend(y);
                return x;
            }
            Term::App(t1, t2) => {
                let mut x = t1.get_bound_vars();
                let y = t2.get_bound_vars();
                x.extend(y);
                return x;
            }
            _ => HashSet::new(),
        }
    }

    pub fn get_free_vars(&self) -> HashSet<Term> {
        match self {
            Term::Var(var) => {
                let mut x: HashSet<Term> = HashSet::new();
                x.insert(Term::Var(*var));
                return x;
            }
            Term::Abs(arg, body) => {
                let mut y = body.get_free_vars();
                let mut x: HashSet<Term> = HashSet::new();
                x.insert(Term::Var(*arg));
                y = y.difference(&x).cloned().collect();
                return y;
            }
            Term::App(t1, t2) => {
                let mut x = t1.get_free_vars();
                let y = t2.get_free_vars();
                x.extend(y);
                return x;
            }
            _ => HashSet::new(),
        }
    }

    pub fn get_vars_that_are_free_and_bound(&self) -> HashSet<Term> {
        let mut x = self.get_free_vars();
        let y = self.get_bound_vars();
        x = x.intersection(&y).cloned().collect();
        return x;
    }

    pub fn create_a_new_var(t1: &Term, t2: &Term) -> Result<u8, Error> {
        let mut vars = t1.get_vars();
        vars.extend(t2.get_vars());
        for i in 0..26 {
            if !vars.contains(&(i + 'a' as u8)) {
                return Ok(i + 'a' as u8);
            }
        }
        Err(Error::NewVariableNotFound)
    }

    // Decides if `var` is free in `self`.
    fn is_free(&self, var: u8) -> bool {
        let free_vars = self.get_free_vars();
        free_vars.contains(&Term::Var(var))
    }

    // Decides if `var` is bound in `self`.
    #[allow(dead_code)]
    fn is_bound(&self, var: u8) -> bool {
        let bound_vars = self.get_bound_vars();
        bound_vars.contains(&Term::Var(var))
    }

    pub fn variable_convention(&self) -> bool {
        for bound in self.get_bound_vars() {
            for free in self.get_free_vars() {
                if free == bound {
                    return false;
                }
            }
        }
        true
    }

    #[allow(dead_code)]
    fn contains_var(&self, var: u8) -> bool {
        match self {
            Term::Var(v) => *v == var,
            Term::Abs(arg, body) => *arg == var || body.contains_var(var),
            Term::App(t1, t2) => t1.contains_var(var) || t2.contains_var(var),
            _ => false,
        }
    }

    fn get_a_new_var(&self) -> Result<u8, Error> {
        for i in 0..26 {
            if !self.contains_var(i + 'a' as u8) {
                return Ok(i + 'a' as u8);
            }
        }
        Err(Error::NewVariableNotFound)
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
            Term::BinOp(_, t1, t2) => t1.replace(var, subs) && t2.replace(var, subs),
        }
    }

    // Reduces `self` if possible. `self` must be mutable. Performs beta reduction mathematically.
    // ((λx M)N) = M[x:=N] (β-Reduction)
    pub fn beta_reduction_(&mut self) -> Result<(), Error> {
        let free_bound_vars = self.get_vars_that_are_free_and_bound();
        match self {
            // beta-reduction
            Term::App(t1, t2) => {
                t2.alpha_conversion(&free_bound_vars)?;
                match &mut **t1 {
                    Term::Abs(var, body) => {
                        if body.replace(*var, t2) {
                            *self = *body.clone();
                        }
                        Ok(())
                    }
                    _ => {
                        t1.beta_reduction_()?;
                        t2.beta_reduction_()?;
                        Ok(())
                    }
                }
            }
            Term::Abs(_, body) => {
                body.beta_reduction_()?;
                Ok(())
            }
            Term::BinOp(op, t1, t2) => {
                t1.beta_reduction_()?;
                t2.beta_reduction_()?;
                match (&**t1, &**t2) {
                    (Term::Constant(c1), Term::Constant(c2)) => match op {
                        Op::Add => {
                            *self = Term::Constant(c1 + c2);
                            return Ok(());
                        }
                        Op::Sub => {
                            *self = Term::Constant(c1 - c2);
                            return Ok(());
                        }
                        Op::Mul => {
                            *self = Term::Constant(c1 * c2);
                            return Ok(());
                        }
                        Op::Div => {
                            *self = Term::Constant(c1 / c2);
                            return Ok(());
                        }
                        Op::Eq => {
                            if c1 == c2 {
                                *self = Term::cond_0();
                                return Ok(());
                            }
                            *self = Term::cond_1();
                            return Ok(());
                        }
                    },
                    _ => Ok(()),
                }
            }
            _ => Ok(()),
        }
    }

    // Creates a reduced `Term` if possible. Performs beta reduction using substitution.
    // ((λx M)N) = M[x:=N] (β-Reduction)
    pub fn beta_reduction(&self) -> Result<Term, Error> {
        match self {
            Term::App(t1, t2) => match &**t1 {
                Term::Abs(arg, body) => Ok(Sub {
                    var: *arg,
                    term1: *t2.clone(),
                    term2: *body.clone(),
                }
                .try_into()?),
                _ => Ok(Term::App(
                    Box::new(t1.beta_reduction()?),
                    Box::new(t2.beta_reduction()?),
                )),
            },
            Term::Abs(arg, body) => Ok(Term::Abs(*arg, Box::new(body.beta_reduction()?))),
            Term::BinOp(op, t1, t2) => {
                let t1 = t1.beta_reduction()?;
                let t2 = t2.beta_reduction()?;
                match (&t1, &t2) {
                    (Term::Constant(c1), Term::Constant(c2)) => match op {
                        Op::Add => Ok(Term::Constant(c1 + c2)),
                        Op::Sub => Ok(Term::Constant(c1 - c2)),
                        Op::Mul => Ok(Term::Constant(c1 * c2)),
                        Op::Div => Ok(Term::Constant(c1 / c2)),
                        Op::Eq => {
                            if c1 == c2 {
                                return Ok(Term::cond_0());
                            }
                            Ok(Term::cond_1())
                        }
                    },
                    _ => Ok(Term::BinOp(op.clone(), Box::new(t1), Box::new(t2))),
                }
            }
            _ => Ok(self.clone()),
        }
    }

    //If y not in FV(M): λx.M = λy.M[x:=y] (α-conversion)
    pub fn alpha_conversion(&mut self, free_bound_vars: &HashSet<Term>) -> Result<(), Error> {
        for free_bound in free_bound_vars {
            if let Term::Var(var) = free_bound {
                if self.is_free(*var) {
                    let new_var = self.get_a_new_var()?;
                    self.replace(*var, &Term::Var(new_var));
                }
            }
        }
        Ok(())
    }

    pub fn create_nested_abs(vs: Vec<u8>, t1: Box<Term>) -> Box<Term> {
        let mut t = t1;
        for var in vs.iter() {
            t = Box::new(Term::Abs(*var, t));
        }
        t
    }

    // Decide if `var` is free in `self`.
    #[allow(dead_code)]
    #[deprecated(note = "use `is_free` instead")]
    fn is_free_(&self, var: u8) -> bool {
        match self {
            Term::Var(var2) => var == *var2,
            Term::Abs(arg, body) => (var != *arg) && body.is_free(var),
            Term::App(t1, t2) => t1.is_free(var) || t2.is_free(var),
            Term::Constant(_) => true,
            _ => false,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Sub {
    //TODO: Only works witth vars. not constants
    pub var: u8,
    pub term1: Term,
    pub term2: Term,
}

impl Sub {
    pub fn to_sub_lippe(&self) -> String {
        format!("Sub({}, {})[{}]", self.var as char, self.term1, self.term2)
    }

    pub fn to_sub(&self) -> String {
        format!("{}[{} := {}]", self.term2, self.var as char, self.term1)
    }
}

impl convert::TryInto<Term> for Sub {
    type Error = Error;

    fn try_into(self) -> Result<Term, Self::Error> {
        let lippe = true; //TODO: Make this a parameter
        match &self.term2 {
            Term::Var(var) => {
                if var == &self.var {
                    Ok(self.term1.clone())
                } else {
                    Ok(self.term2.clone())
                }
            }
            Term::Constant(_) => Ok(self.term2.clone()),
            Term::App(t1, t2) => Ok(Term::App(
                Box::new(
                    Sub {
                        var: self.var,
                        term1: self.term1.clone(),
                        term2: t1.as_ref().clone(),
                    }
                    .try_into()?,
                ),
                Box::new(
                    Sub {
                        var: self.var,
                        term1: self.term1.clone(),
                        term2: t2.as_ref().clone(),
                    }
                    .try_into()?,
                ),
            )),
            Term::Abs(arg, body) => {
                if arg == &self.var {
                    Ok(self.term2.clone())
                } else if lippe {
                    if !self.term1.get_free_vars().contains(&Term::Var(arg.clone()))
                        || !body.get_free_vars().contains(&Term::Var(self.var))
                    {
                        Ok(Term::Abs(
                            arg.clone(),
                            Box::new(
                                Sub {
                                    var: self.var,
                                    term1: self.term1.clone(),
                                    term2: *body.clone(),
                                }
                                .try_into()?,
                            ),
                        ))
                    } else {
                        let new_var = Term::create_a_new_var(&self.term1, &body)?;
                        Ok(Term::Abs(
                            new_var,
                            Box::new(
                                Sub {
                                    var: self.var,
                                    term1: self.term1.clone(),
                                    term2: Sub {
                                        var: *arg,
                                        term1: Term::Var(new_var),
                                        term2: *body.clone(),
                                    }
                                    .try_into()?,
                                }
                                .try_into()?,
                            ),
                        ))
                    }
                } else {
                    Ok(Term::Abs(
                        arg.clone(),
                        Box::new(
                            Sub {
                                var: self.var,
                                term1: self.term1.clone(),
                                term2: body.as_ref().clone(),
                            }
                            .try_into()?,
                        ),
                    ))
                }
            }
            _ => Ok(self.term2.clone()),
        }
    }
}
