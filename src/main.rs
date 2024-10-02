#![allow(unused)]

mod parser;

use parser::*;


fn main() {
    use crate::parser::*;

    let negation = Rule {
        head: parse_expression("~(~A)").unwrap(),
        tail: parse_expression("A").unwrap(),
    };
    let random_rule = Rule {
        head: parse_expression("~(A & B)").unwrap(),
        tail: parse_expression("~A | ~B").unwrap()
    };
    let expression = parse_expression("~(~((A | D) & ~C))").unwrap();

    println!("negation: {negation}");
    println!("random_rule: {random_rule}");
    println!("expr: {expression}");
    println!("----------------Matching----------------");

    if let Some(matches) = find_match(&random_rule, &expression) {
        for m in matches.iter() {
            println!("{m}")
        }
    } else {
        println!("No Match. rule: {random_rule}, expr: {expression}")
    }

}
