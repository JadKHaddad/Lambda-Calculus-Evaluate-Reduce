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
    reduce: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce_steps: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    print_bound_variables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    print_free_variables: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable_convention: Option<bool>,
}

fn main() {
    let mut term = term::TermsParser::new()
    .parse("(λx . xx)((λx . x)(λx . x))")
    .expect(&format!("Unable to parse term"));
    let mut term_2 = term.clone();
    term.beta_reduction_(true).unwrap();
    term_2.beta_reduction_(false).unwrap();
    println!("outer reduction: {}", term);
    println!("inner reduction: {}", term_2);
    // (λr (x(λk k)))(λz (zy))
    // let term = term::TermsParser::new()
    // .parse("(λy (x(λx x)))(λz (zy))")
    // .expect(&format!("Unable to parse term"));
    // println!("{}", term);
    // println!("{}", term.variable_convention());
    // //let t = term.alpha_conversion(b'q').unwrap();
    // let t = term.alpha_conversion(b'q').unwrap().alpha_conversion(b'z').unwrap();
    // println!("{t}");
    // println!("{}", term.variable_convention());
    std::process::exit(0);
    let files = ["terms.yaml", "strict.yaml", "dynamic.yaml"];
    for file in files {
        println!("------------ {} ------------", file);
        let input = fs::read_to_string(file).expect("Unable to read file");
        let terms: Vec<InputTerm> = serde_yaml::from_str(&input).expect("Unable to parse YAML");
        for input_term in terms {
            println!("----------------------------");
            let mut term = if file == "terms.yaml" {
                term::TermsParser::new()
                    .parse(&input_term.term)
                    .expect(&format!("Unable to parse term: [{}]", input_term.term))
            } else if file == "strict.yaml" {
                term::StrictParser::new()
                    .parse(&input_term.term)
                    .expect(&format!("Unable to parse term: [{}]", input_term.term))
            } else if file == "dynamic.yaml" {
                println!("Parsing with TermsParser");
                match term::TermsParser::new().parse(&input_term.term) {
                    Ok(term) => term,
                    Err(_) => {
                        println!("Parsing with StrictParser");
                        term::StrictParser::new()
                            .parse(&input_term.term)
                            .expect(&format!("Unable to parse term: [{}]", input_term.term))
                    }
                }
            } else {
                panic!("Unknown file: {}", file);
            };
            println!("Term: [ {} ]", term);
            if input_term.print_ast.unwrap_or(false) {
                println!("AST: {:?}", term);
            }
            if input_term.print_bound_variables.unwrap_or(false) {
                println!(
                    "Bound Variables: {:?}",
                    term.get_bound_vars()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                );
            }
            if input_term.print_free_variables.unwrap_or(false) {
                println!(
                    "Free Variables: {:?}",
                    term.get_free_vars()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                );
            }
            if input_term.variable_convention.unwrap_or(false) {
                println!("Variable Convention: {}", term.variable_convention());
            }
            if input_term.reduce.unwrap_or(false) {
                let mut betta_term = *term.clone();
                if let Some(reduce_steps) = input_term.reduce_steps {
                    for _ in 0..reduce_steps {
                        match term.beta_reduction_(true){
                            Ok(()) => {
                                println!("Reduced: [ {} ] | mathematically ", term);
                            },
                            Err(e) => {
                                println!("Error: {} | mathematically", e);
                            }
                        }

                        betta_term = betta_term.beta_reduction().unwrap();

                        println!("Reduced: [ {} ] | using substitution", betta_term);
                    }
                } else {
                    match term.beta_reduction_(true){
                        Ok(()) => {
                            println!("Reduced: [ {} ] | mathematically ", term);
                        },
                        Err(e) => {
                            println!("Error: {} | mathematically", e);
                        }
                    }
                    println!(
                        "Reduced: [ {} ] | using substitution",
                        betta_term.beta_reduction().unwrap()
                    );
                }
            }
        }
        println!("----------------------------");
    }
}
