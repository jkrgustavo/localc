#![allow(unused)]

use std::fmt::Formatter;
use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;


/*          Expression
*
*===============================
*/

#[derive(Debug, PartialEq, Hash, Clone)]
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

// Test only the top level expression
impl Expr {
    pub fn shallow_eq(&self, rhs: &Expr) -> bool { 
        match (self, rhs) {
            (Expr::Sym(a), Expr::Sym(b)) => a == b,
            (Expr::Not(_), Expr::Not(_)) => true,
            (Expr::And(_, _), Expr::And(_, _)) => true,
            (Expr::Or(_, _), Expr::Or(_, _)) => true,
            _ => false
        }   
    }
}

// Find instances of a rule within an expression
impl Expr {
    pub fn find_match(&self, rule: &Rule) -> Option<Vec<Match>> {
        let mut matches = Vec::new();
        match self.find_trav(rule, &mut matches) {
            Some(_) => Some(matches),
            None => None
        }
    }

    pub fn find_trav(&self, rule: &Rule, mats: &mut Vec<Match>) -> Option<()> {
        use Expr::*;

        if rule.head.shallow_eq(self) {
            let mut mat = Match::new(self.clone(), rule.clone());

            match self.find_impl(&rule.head, &mut mat.binds) {
                Some(_) => return Some(mats.push(mat)),
                None => {}
            }
        }

        match self {
            Not(e) => e.find_trav(rule, mats),
            And(l, r) => l.find_trav(rule, mats).or(r.find_trav(rule, mats)),
            Or(l, r) => l.find_trav(rule, mats).or(r.find_trav(rule, mats)),
            _ => { None }
        }
    }

    pub fn find_impl(&self, rule: &Expr, bindings: &mut Bindings) -> Option<()> {
        use Expr::*;

        match (rule, self) {
            (Sym(name), _) => { 
                bindings
                    .entry(name.clone())
                    .or_insert(self.clone());
                Some(())
            },

            (Not(r), Not(e)) => e.find_impl(r, bindings),

            (And(l_rule, r_rule), And(l_expr, r_expr)) => { 
                l_expr.find_impl(l_rule, bindings)?;
                r_expr.find_impl(r_rule, bindings)?;
                Some(())
            },
            (Or(l_rule, r_rule), Or(l_expr, r_expr)) => { 
                l_expr.find_impl(l_rule, bindings)?;
                r_expr.find_impl(r_rule, bindings)?;
                Some(())
            },
            _ => { None }
        }
    }
}

// Apply rules to an expression
impl Expr {
    pub fn apply_rule(&self, rule: &Rule) -> Result<Expr, String> {
        let matches = self.find_match(rule)
            .ok_or_else(|| format!("Unable to find instances of rule [[{rule}]] in [[{self}]]"))?;

        matches.into_iter().try_fold(self.clone(), |acc, mat| acc.apply_trav(&mat))
    }


    fn apply_trav(&self, mat: &Match) -> Result<Expr, String> {
        use Expr::*;

        if mat.full_expr == *self {
            return mat.rule.tail.clone().apply_impl(mat);
        }

        match self {
            Sym(s) => Ok(Sym(s.clone())),
            Not(e) => Ok(Not(Box::new(e.apply_trav(mat)?))),
            And(l, r) => Ok(And(
                Box::new(l.apply_trav(mat)?), 
                Box::new(r.apply_trav(mat)?)
            )),
            Or(l, r) => Ok(Or(
                Box::new(l.apply_trav(mat)?), 
                Box::new(r.apply_trav(mat)?)
            )),
        }
    }

    fn apply_impl(&self, mat: &Match) -> Result<Expr, String> {
        use Expr::*;

        match self {
            Sym(s) => Ok(mat.binds.get(s)
                .cloned()
                .ok_or_else(|| format!("Unknown binding: '{s}'"))?
            ),
            Not(e) => Ok(Not(Box::new(e.apply_impl(mat)?))),
            And(l, r) => Ok(And(
                Box::new(l.apply_impl(mat)?), 
                Box::new(r.apply_impl(mat)?)
            )),
            Or(l, r) => Ok(Or(
                Box::new(l.apply_impl(mat)?), 
                Box::new(r.apply_impl(mat)?)
            )),
        }
    }
    
}


/*            Rule
*
*===============================
*/

#[derive(Debug, Clone)]
pub struct Rule {
    pub head: Expr,
    pub tail: Expr,
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f, "{} => {}", self.head, self.tail)
    }
}

/*          Match
*
*===============================
*/

type Bindings = HashMap<String, Expr>;

#[derive(Debug)]
pub struct Match {
    binds: Bindings,
    full_expr: Expr,
    rule: Rule
}

impl Match {
    pub fn new(expr: Expr, rule: Rule) -> Match {
        Match {
            binds: HashMap::new(),
            full_expr: expr,
            rule
        }
    }
}

impl std::fmt::Display for Match {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        writeln!(f, "{}", self.rule)?;
        writeln!(f, "{}", self.full_expr)?;

        let mut iter = self.binds.iter().peekable();
        while let Some((k, v)) = iter.next() {
            if iter.peek().is_some() {
                writeln!(f, "    '{k}' -> {v}")?;
            } else {
                write!(f, "    '{k}' -> {v}")?;
            }
        }
        Ok(())
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


