#![allow(unused)]

mod parser;
use crate::parser::*;

fn main() -> Result<(), String> {

    let negation = Rule {
        head: parse_expression("~(~A)").unwrap(),
        tail: parse_expression("A").unwrap(),
    };
    let de_morgans = Rule {
        head: parse_expression("~(A & B)").unwrap(),
        tail: parse_expression("~A | ~B").unwrap()
    };
    let expression = parse_expression("(~((A | D) & ~(~C)))").unwrap();

    println!("[Expression]: {expression}");
    println!("[Negation]:   {negation}");
    println!("[DeMorgan]:   {de_morgans}");
    println!("{}", expression
        .apply_rule(&negation)?
        .apply_rule(&de_morgans)?
    );

    Ok(())
}

