use crate::ast::{Address, Node};
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Chip8ASMParser;

// This is a good place to use the type state pattern instead
// raw input -> macros expanded -> symbol table built -> final AST
pub(crate) fn parse(input: &mut str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];

    let mut labels = build_symbols(input);

    let pairs = Chip8ASMParser::parse(Rule::Program, input)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair, &mut labels));
        }
    }

    Ok(ast)
}

fn expand_macros(input: &mut str) {}

fn build_symbols(input: &mut str) -> HashMap<String, Address> {
    todo!()
}

fn build_ast_from_expr(pair: Pair<Rule>, labels: &mut HashMap<String, Address>) -> Node {
    match pair.as_rule() {
        Rule::NullExpr => {}
        Rule::UnaryExpr => {}
        Rule::BinaryExpr => {}
        Rule::TernaryExpr => {}
        Rule::Label => {}
        _ => {}
    }

    todo!()
}
