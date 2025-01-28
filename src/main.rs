use std::env;
use std::fs;

use self::AstNode::*;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hana.pest"]
pub struct HanaParser;

#[derive(Debug)]
pub enum AstNode {
    Print(Box<AstNode>),
    Integer(i32),
    Real(f64),
    Ident(String),
    Sexpr {
        params: Vec<Box<AstNode>>,
    },
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = HanaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::sexpr => {
                ast.push(build_ast_from_sexpr(pair));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn build_ast_from_sexpr(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::integer => {

            println!("integer pair? {:?}", pair);
            Integer(0)

             }
        Rule::real => { Real(0.0)}
        Rule::string => { Integer(0) }
        Rule::ident => { Ident(String::from(pair.as_str())) }
        Rule::sexpr => {

            while let Some(p) = pair.into_inner().next() {
                println!("{:?}", p);
            }
            build_ast_from_sexpr(pair.into_inner().next().unwrap())
        }
        _ => {Integer(0)}
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let unparsed_file = fs::read_to_string("src/test.hana").expect("cannot read file!");

    // let file = HanaParser::parse(Rule::program, &unparsed_file)
    //     .expect("unsuccessful parse")
    //     .next()
    //     .unwrap();

    let file = parse(&unparsed_file)
        .expect("unsuccessful parse");

    println!("{:?}", file);
}
