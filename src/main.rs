pub mod ast;
pub mod term;
fn main() {
    let term = term::TermParser::new()
        .parse("(λx. (λx. f(x)))(x)((λx. x)(z))")
        .unwrap();
    println!("{:?}", term);
    println!("{}", term);
}
