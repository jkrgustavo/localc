# Propositional Logic Calculator

Very simple parser for logical expressions. It allows you to create simple logical expressions,
then define rules and apply them to that expression.

For example you could create a simple expression like so:
```rust
    let expression = parse_expression("(~((A | D) & ~(~C)))").unwrap();
```
then define a simple rule as well:
```rust
    let double_negation = Rule {
        head: parse_expression("~(~A)").unwrap(),
        tail: parse_expression("A").unwrap(),
    };
```
Where 'head' is what will be looked for, and 'tail' is what it'll be replaced with. Call the 
`apply_rule` function on the expression to create a new expression with all instances of 'head'
replaced with 'tail':
```rust
    let modified_expression = expression.apply_rule(&negation);
```
Because the `apply_rule()` function returns an expression, you can use chaining to apply
several expressions in sucession:
```rust
    let modified_expression = expression
        .apply_rule(&negation)?
        .apply_rule(&de_morgans);
```

**Supported operators**:
- Or: ∨, |
- And: ∧, &
- Not: ¬, ~

