#![allow(unused)]

use std::fmt::Formatter;
use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;


/*          Expression
*
*===============================
*/

#[derive(Debug, Eq, Hash, Clone)]
pub enum Expr {
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Sym(String)
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            Expr::Sym(s) => write!(f, "{}", s),
            Expr::Not(exp) => write!(f, "¬{}", exp),
            Expr::And(ex1, ex2) => write!(f, "({} ∧ {})", ex1, ex2),
            Expr::Or(ex1, ex2) => write!(f, "({} ∨ {})", ex1, ex2),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, rhs: &Expr) -> bool { 
        match (self, rhs) {
            (Expr::Sym(a), Expr::Sym(b)) => a == b,
            (Expr::Not(_), Expr::Not(_)) => true,
            (Expr::And(_, _), Expr::And(_, _)) => true,
            (Expr::Or(_, _), Expr::Or(_, _)) => true,
            _ => false
        }   
    }
}


/*            Rule
*
*===============================
*/

#[derive(Debug)]
pub struct Rule {
    pub head: Expr,
    pub tail: Expr,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{} => {}", self.head, self.tail)
    }
}

type Bindings = HashMap<String, Vec<Expr>>;

pub fn find_match(rule: &Rule, expr: &Expr) -> Option<Bindings> {
    let mut bindings = HashMap::new();
    match find_rule(&rule, expr, &mut bindings) {
        Some(_) => Some(bindings),
        None => None
    }
}

fn find_rule_impl(rule: &Expr, expr: &Expr, bindings: &mut Bindings) -> Option<()> {
    use crate::parser::Expr::*;

    match (rule, expr) {
        (Sym(name), _) => { 
            bindings
                .entry(name.clone())
                .or_insert_with(Vec::new)
                .push(expr.clone());
            Some(())
        },

        (Not(r), Not(e)) => find_rule_impl(&r, &e, bindings),

        (And(l_rule, r_rule), And(l_expr, r_expr)) => { 
            find_rule_impl(l_rule, l_expr, bindings)?;
            find_rule_impl(r_rule, r_expr, bindings)?;
            Some(())
        },
        (Or(l_rule, r_rule), Or(l_expr, r_expr)) => { 
            find_rule_impl(l_rule, l_expr, bindings)?;
            find_rule_impl(r_rule, r_expr, bindings)?;
            Some(())
        },
        _ => { None }
    }
}

fn find_rule(rule: &Rule, expr: &Expr, bindings: &mut Bindings) -> Option<()> {
    use Expr::*;

    if &rule.head == expr {
        return find_rule_impl(&rule.head, expr, bindings);
    }

    match expr {
        Not(e) => find_rule(rule, &e, bindings),
        And(l, r) => find_rule(rule, l, bindings).or(find_rule(rule, r, bindings)),
        Or(l, r) => find_rule(rule, l, bindings).or(find_rule(rule, r, bindings)),
        _ => { None }
    }

}

/*          Parser
*
*===============================
*/

struct Parser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {

    fn new(input: &'a str) -> Self {
        Parser {
            input: input.chars().peekable(),
        }
    }

    fn parse(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.consume_next('|') {
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }


    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        while self.consume_next('&') {
            let right = self.parse_unary()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.consume_next('~') {
            let mut left = self.parse_atom()?;
            Ok(Expr::Not(Box::new(left)))
        } else {
            self.parse_atom()
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();
        match self.input.peek() {
            Some('(') => {
                self.input.next();
                let expr = self.parse_or()?;
                if !self.consume_next(')') {
                    return Err("Unmatched parenthese!!!".to_string())
                }
                Ok(expr)

            }
            Some(c) if c.is_alphabetic() => {
                let mut symbol = String::new();
                while let Some(&c) = self.input.peek() {
                    if c.is_alphabetic() {
                        self.input.next();
                        symbol.push(c);
                    } else {
                        break;
                    }
                }
                Ok(Expr::Sym(symbol))
            }
            Some(c) => Err(format!("Unexpected symbol: {c}")),
            None => Err("None branch!".to_string())
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn consume_next(&mut self, expected: char) -> bool {
        self.skip_whitespace();
        if self.input.peek() == Some(&expected) {
            self.input.next();
            true
        } else {
            false
        }
    }

}

pub fn parse_expression<'a>(expr: &'a str) -> Result<Expr, String> {
    Parser::new(expr).parse()
}
