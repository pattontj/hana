use std::env;
use std::fs;

use self::AstNode::*;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hana.pest"]
pub struct HanaParser;

pub enum AstNode {
    Print(Box<AstNode>),
    Integer(i32),
    DoublePrecisionFloat(f64),
    Sexpr {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = HanaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::sexpr => {
                ast.push(Print(Box::new(build_ast_from_sexpr(pair))));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn build_ast_from_sexpr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {}
        Rule::real => {}
        Rule::string => {}
        Rule::ident => {}
        Rule::sexpr => {}
        _ => {}
    }
    Integer(0)
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let unparsed_file = fs::read_to_string("src/test.hana").expect("cannot read file!");

    let file = HanaParser::parse(Rule::program, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    println!("{:?}", file);
}
