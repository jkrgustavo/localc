#![allow(unused)]

mod parser;

use parser::*;


fn main() {
    use crate::parser::*;

    let rule = Rule {
        head: parse_expression("~(~A)").unwrap(),
        tail: parse_expression("A").unwrap(),
    };

    let expression = parse_expression("(~(~A) | B) & ~(~(C | D))").unwrap();

    println!("rule: {rule}");
    println!("expr: {expression}");

    match find_match(&rule, &expression) {
        Some(m) => println!("{:?}", m),
        None => println!("No match")
    }
}
