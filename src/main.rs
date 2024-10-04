#![allow(unused)]

mod parser;

use parser::*;


fn main() -> Result<(), String> {
    use crate::parser::*;

    let negation = Rule {
        head: parse_expression("~(~A)").unwrap(),
        tail: parse_expression("A").unwrap(),
    };
    let random_rule = Rule {
        head: parse_expression("~(A & B)").unwrap(),
        tail: parse_expression("~A | ~B").unwrap()
    };
    let expression = parse_expression("(~((A | D) & ~(~C)))").unwrap();

    //println!("negation: {negation}\nrandom_rule: {random_rule}\nexpr: {expression}");
    println!("----------------Matching----------------");

    if let Some(v) = expression.find_match(&random_rule) {
        //println!("Rule: {random_rule}");
        for m in v.iter() {
            println!("{m}")
        }
    }

    println!("----------------Applying----------------");
    println!("Expr: {expression}");
    println!("Rules: {negation} -- {random_rule}");
    println!("{}", expression
        .apply_rule(&negation)?
        .apply_rule(&random_rule)?
    );


    Ok(())
}

