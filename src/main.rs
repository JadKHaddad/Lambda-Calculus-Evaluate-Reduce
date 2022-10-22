pub mod ast;
pub mod term;
fn main() {
    println!("----------------------------");
    let term = term::TermParser::new().parse("(λz. (x x))").unwrap();
    println!("Term: [ {} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new().parse("(λxyzu. (x x))").unwrap();
    println!("Term: [ {} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("(λa (λb (λc (x x))))")
        .unwrap();
    println!("Term: [ {} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("((λx (x * ((λx (x + ((λx x) 3))) 8))) 3)")
        .unwrap();
    println!("Term: [ {} ] = [ {} ]", term, term.evaluate());
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("((λx ((λy (x * y)) 4)) 3)")
        .unwrap();
    println!("Term: [ {} ] = [ {} ]", term, term.evaluate());
    println!("Term: [ {:?} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("((λx ((λy (x + y)) 8)) 100)")
        .unwrap();
    println!("Term: [ {} ] = [ {} ]", term, term.evaluate());
    println!("----------------------------");
    let mut term = term::TermParser::new()
        .parse("(((λx (λx (f x))) x) ((λx x) z))")
        .unwrap();
    let term_temp = term.clone();
    println!("Term: [ {} ]", term_temp);
    term.reduce();
    println!("Term: [ {} ] (After reduction) = [ {} ]", term_temp, term);
    term.reduce();
    println!(
        "Term: [ {} ] (After deep reduction) = [ {} ]",
        term_temp, term
    );
    println!("----------------------------");
    let mut term = term::TermParser::new()
        .parse("((((λx (x y)) (λz (x z))) f) (g h))")
        .unwrap();
    let term_temp = term.clone();
    println!("Term: [ {} ]", term_temp);
    term.reduce();
    println!("Term: [ {} ] (After reduction) = [ {} ]", term_temp, term);
    term.reduce();
    println!(
        "Term: [ {} ] (After deep reduction) = [ {} ]",
        term_temp, term
    );
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("((λx((λy((x+y)*y))3))4)")
        .unwrap();
    println!("Term: [ {} ] = [ {} ]", term, term.evaluate());
    println!("Term: [ {:?} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("(((ux)(yz))(λv(vy)))")
        .unwrap();
    println!("Term: [ {} ]", term);
    println!("Term: [ {:?} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("((((λx(λy(λz((xz)(yz)))))u)v)w)")
        .unwrap();
    println!("Term: [ {} ]", term);
    println!("Term: [ {:?} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new()
        .parse("(((w(λx(λy(λz((xz)(yz))))))u)v)")
        .unwrap();
    println!("Term: [ {} ]", term);
    println!("Term: [ {:?} ]", term);
    println!("----------------------------");
    let term = term::TermParser::new().parse("((λx(xx))(λx(xx)))").unwrap();
    println!("Term: [ {} ]", term);
    println!("Term: [ {:?} ]", term);
    println!("----------------------------");
}
