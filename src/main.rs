use std::env;
use std::ffi::CString;
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

    Str(CString),

    Ident(String),

    Sexpr { params: Vec<Box<AstNode>> },
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
            // println!("integer pair? {:?}", pair);
            let i: i32 = pair.as_str().parse().unwrap();
            Integer(i)
        }
        Rule::real => {
            let r: f64 = pair.as_str().parse().unwrap();
            Real(r)
        }
        Rule::string => {
            let str = &pair.as_str();
            // println!("{}", str);
            let str = &str[1..str.len() - 1];
            // println!("{}", str);
            // should be: \" -> "
            let str = str.replace("\\\"", "\"");

            Str(CString::new(&str[..]).unwrap())
        }
        Rule::ident => Ident(String::from(pair.as_str())),
        Rule::sexpr => {
            let mut pair = pair.into_inner();

            let mut params = Vec::new();

            while let Some(p) = pair.next() {
                // println!("{:?}", p);
                params.push(Box::new(build_ast_from_sexpr(p)));
            }

            Sexpr { params }
        }
        _ => Integer(0),
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let unparsed_file = fs::read_to_string("src/test.hana").expect("cannot read file!");

    // let file = HanaParser::parse(Rule::program, &unparsed_file)
    //     .expect("unsuccessful parse")
    //     .next()
    //     .unwrap();

    let file = parse(&unparsed_file).expect("unsuccessful parse");

    println!("{:?}", file);
}
