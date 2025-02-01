use std::borrow::Borrow;
// use std::borrow::{Borrow, BorrowMut};
// use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::rc::Rc;

use self::AstNode::*;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hana.pest"]
pub struct HanaParser;

#[derive(Debug, Clone)]
pub enum AstNode {
    Integer(i32),
    Real(f64),

    Str(CString),

    Symbol(String),

    Nil(),

    List { elements: Vec<Box<AstNode>> },
}

pub fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = HanaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::form => {
                ast.push(build_ast_from_form(pair));
            }
            _ => {}
        }
    }

    Ok(ast)
}

fn build_ast_from_form(pair: pest::iterators::Pair<Rule>) -> AstNode {
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
        Rule::symbol => Symbol(String::from(pair.as_str())),

        Rule::list => {
            let mut pair = pair.into_inner();

            let mut elements = Vec::new();

            while let Some(p) = pair.next() {
                // println!("{:?}", p);
                elements.push(Box::new(build_ast_from_form(p)));
            }

            List { elements }
        }

        Rule::form => build_ast_from_form(pair.into_inner().next().unwrap()),
        _ => Nil(),
    }
}

type Context = HashMap<String, Rc<RefCell<AstNode>>>;

pub trait ContextExt {
    fn bind_symbol(&mut self, symbol: String, value: AstNode);
    fn lookup_symbol(&self, symbol: String) -> Option<&Rc<RefCell<AstNode>>>;
}

impl ContextExt for Context {
    fn bind_symbol(&mut self, symbol: String, value: AstNode) {
        match value {
            AstNode::Symbol(value) => {
                if let Some(lookup) = self.lookup_symbol(value.clone()) {
                    println!(
                        "Found binding from symbol {:?} to value {:?}",
                        symbol, lookup
                    );
                    let symref = Rc::clone(lookup);
                    self.insert(symbol, symref);
                } else {
                    println!("Error: could not find a binding with symbol {:?}", symbol);
                }
            }
            _ => {
                self.insert(symbol, Rc::new(RefCell::new(value)));
            }
        }
    }

    fn lookup_symbol(&self, symbol: String) -> Option<&Rc<RefCell<AstNode>>> {
        self.get(&symbol)
    }
}

pub struct Environment {
    bindings: Vec<Context>,
}

impl Environment {
    fn bind_symbol(&mut self, symbol: String, value: AstNode) {
        let len = self.bindings.len();
        let ctx = self.bindings.get_mut(len - 1).unwrap();

        ctx.bind_symbol(symbol, value);
    }
    fn lookup_symbol(&self, symbol: String) -> Option<Rc<RefCell<AstNode>>> {
        let mut ctx = self.bindings.iter();

        while let Some(ctx) = ctx.next() {
            let found = ctx.lookup_symbol(symbol);
            if let Some(found) = found {
                return Some(Rc::clone(found));
            } else {
                return None;
            }
        }

        None
    }
}

pub fn evaluate(form: AstNode, env: &Environment) -> AstNode {
    match form {
        AstNode::Integer(_) => form,

        AstNode::Real(_) => form,

        AstNode::Str(_) => form,

        AstNode::Symbol(form) => {
            let mut result = env.lookup_symbol(form as String);

            if result.is_some() {
                // let test = result.unwrap().borrow_mut().to_owned();
                let x = result.as_mut().unwrap();
                let tmp = x.as_ref();
                println!("reading from rc refcell? {:?}", tmp);
                // println!("reading from rc refcell? {:?}", i32::from(tmp.into_inner()));
                // println!(
                //     "reading from rc refcell? {:?}",
                //     *result.unwrap().borrow_mut()
                // );
            }

            Nil()
        }

        _ => Nil(),
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let unparsed_file = fs::read_to_string("tests/test.hana").expect("cannot read file!");

    // let file = HanaParser::parse(Rule::program, &unparsed_file)
    //     .expect("unsuccessful parse")
    //     .next()
    //     .unwrap();

    let file = parse(&unparsed_file).expect("unsuccessful parse");

    // let context = Context {
    //     bindings: HashMap::new(),
    // };

    let mut env = Environment {
        bindings: vec![Context::new()],
    };

    env.bind_symbol("x".to_string(), Integer(2));
    env.bind_symbol("y".to_string(), Symbol("x".to_string()));

    let mut result = Nil();
    for form in file {
        result = evaluate(form, &env);
    }

    println!("{:?}", result);
}
