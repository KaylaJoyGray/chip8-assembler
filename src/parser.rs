use crate::ast::Node;
use pest::iterators::Pair;
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Chip8ASMParser;

pub(crate) fn parse(input: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = Chip8ASMParser::parse(Rule::Program, input)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }

    Ok(ast)
}

fn build_ast_from_expr(pair: Pair<Rule>) -> Node {
    todo!()
}
