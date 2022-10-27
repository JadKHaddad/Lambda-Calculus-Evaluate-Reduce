/*
Will parse every Lambda-calculus term as long as it is (right) parenthesized.
Not-Parenthesized terms are only allowed as an abstraction with the Dot operator. e.g. [λx . x], [λx . (λy . (x y))]
Summery: Not-Parenthesized terms and Dot operator are not well supported.
*/
use serde::{Deserialize, Serialize};
use std::fs;
pub mod ast;
pub mod term;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct InputTerm {
    term: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    print_ast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evaluate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce_steps: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Input {
    terms: Vec<InputTerm>,
}

fn main() {
    // let term = term::SParser::new().parse("((λx ((yz)(λx(λy(yx)))))z)").unwrap();
    // println!("Term: {}", term);
    // println!("BV: {:?}", term.get_bound_vars().iter().map(|x| x.to_string()).collect::<Vec<String>>());
    // println!("FV: {:?}", term.get_free_vars().iter().map(|x| x.to_string()).collect::<Vec<String>>());
    // println!("----------------------------");
    // let term = term::SParser::new().parse("((xz)(λx(λyx)))").unwrap();
    // println!("Term: {}", term);
    // println!("BV: {:?}", term.get_bound_vars().iter().map(|x| x.to_string()).collect::<Vec<String>>());
    // println!("FV: {:?}", term.get_free_vars().iter().map(|x| x.to_string()).collect::<Vec<String>>());
    // println!("----------------------------");
    let sub = ast::Sub {
        var: 'y' as u8,
        term1: *term::SParser::new().parse("(λx(λy x))").unwrap(),
        term2: *term::SParser::new().parse("(y(λx(yx)))").unwrap(),
    };
    println!("Sub lisp: {}", sub.to_sub_lisp());
    println!("Sub: {}", sub.to_sub());
    let t = sub.create_term();
    println!("Term: {}", t);
    std::process::exit(0);


    let input = fs::read_to_string("terms.yaml").expect("Unable to read file");
    let input: Input = serde_yaml::from_str(&input).expect("Unable to parse YAML");
    for input_term in input.terms {
        println!("----------------------------");
        let mut term = term::SParser::new().parse(&input_term.term).unwrap();
        println!("Term: [ {} ]", term);

        if input_term.print_ast.unwrap_or(false) {
            println!("AST: {:?}", term);
        }
        if input_term.evaluate.unwrap_or(false) {
            println!("Evaluated: [ {} ]", term.evaluate());
        }
        if input_term.reduce.unwrap_or(false) {
            if let Some(reduce_steps) = input_term.reduce_steps {
                for _ in 0..reduce_steps {
                    term.reduce();
                    println!("Reduced: [ {} ]", term);
                }
            } else {
                term.reduce();
                println!("Reduced: [ {} ]", term);
            }
        }
    }
    println!("----------------------------");
}
