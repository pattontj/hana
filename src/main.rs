use std::borrow::{Borrow, BorrowMut};
// use std::borrow::{Borrow, BorrowMut};
// use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::iter::zip;
use std::ops::DerefMut;
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

    List {
        elements: Vec<Box<AstNode>>,
    },
    Function {
        params: Vec<AstNode>,
        context: Context,
        body: Box<AstNode>,
    },
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
                        value, lookup
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

pub fn evaluate(form: AstNode, env: &mut Environment) -> AstNode {
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
                // println!("reading from rc refcell? {:?}", tmp);
                // println!("reading from rc refcell? {:?}", i32::from(tmp.into_inner()));
                // println!(
                //     "reading from rc refcell? {:?}",
                //     *result.unwrap().borrow_mut()
                // );

                let test = tmp.borrow_mut().clone();
                // println!("cloned refcell: {:?}", test);
                return test;
            }

            Nil()
        }

        AstNode::List { elements } => {
            /*
            List Evaluation Order:
            1. Special-Form
                A special form is a list whose first element is a symbol that matches
                a pre-defined set of names, set aside by the language for special use.
                Each S-Form has a non-standard behaviour and evaluation protocol, hence
                the name "special".

            2. Macro Call [NOT-IMPLEMENTED]
                Macros are not yet supported.

            3. Function Call
                A function call is when the evaluating list's first element is a symbol
                that matches a defined function. Every following element of the list
                is considered to be arguments to the function.
            */

            /*
                Function Call Implementation:
                ======================================
                Check if list[0] is a symbol,
                if no:
                    fail
                else:
                    lookup = evaluate(list[0])

                    Check if lookup is a lambda,
                    if yes:
                        1. create a new context,
                        2. bind args to formal params in that new context,
                        3. Parse lambda-body list for non-param symbols
                            and bind any found to the new context
                        4. move new context into the environment,
                        5. evaluate the lambda

                    if no:
                        fail
            */

            // if it's an empty list, return nil
            if elements.len() == 0 {
                return Nil();
            }

            let first = *elements[0].clone();

            // fetch the first element, check if it's a symbol,
            // if it is then bind the args to the function's parameters
            match first {
                Symbol(first) => {
                    if let Some(lookup) = env.lookup_symbol(first) {
                        println!("Lookup: {:?}", lookup);

                        let fun = lookup.as_ref().borrow_mut().clone();

                        match fun {
                            Function {
                                params,
                                context,
                                body,
                            } => {
                                println!("[DEBUG] Valid function form");

                                println!("\tparams: {params:?}");
                                println!("\tcontext: {context:?}");
                                println!("\tbody: {body:?}");
                                println!("\n");

                                // create a new context, and for each formal parameter, bind the args passed
                                // in the form being evaluated

                                let ctx = Context::new();

                                // IDEA: get iterators for params and the form being evaluated instead of janky for loops

                                println!("\t=Matching param to args=");
                                if let Some((f, rest)) = elements.as_slice().split_first() {
                                    for (param, arg) in zip(params, rest) {
                                        println!("param={param:?}, arg={arg:?}");
                                        match param {
                                            Symbol(param) => {
                                                env.bind_symbol(param, *arg.clone());
                                            }

                                            _ => {
                                                println!("Error: formal parameter {param:?} is not a symbol");
                                            }
                                        }
                                    }
                                }

                                // for param in params {
                                //     println!("parameter: {param:?}");
                                //     println!("parameter: {param:?}");
                                // }

                                return evaluate(*body, env);
                            }
                            _ => {
                                println!("Error: evaluated list is not a valid form");
                            }
                        }
                    }
                }
                _ => {
                    println!("Error: evaluated list is not a valid form");
                }
            }

            for elem in elements {
                let res = evaluate(*elem, env);
                println!("evaluated: {res:?}");
            }
            // println!("elements: {:?}", elements);
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

    let env = &mut Environment {
        bindings: vec![Context::new()],
    };

    env.bind_symbol("x".to_string(), Integer(2));
    env.bind_symbol("y".to_string(), Symbol("x".to_string()));

    // (lambda (x) (+ 2 x))
    let f1 = Function {
        params: vec![Symbol("x".to_string())],
        context: Context::new(),
        body: Box::new(List {
            elements: vec![
                Box::new(Symbol("+".to_string())),
                Box::new(Integer(2)),
                Box::new(Symbol("x".to_string())),
            ],
        }),
    };

    // (lambda () "Hello, hana!")
    let f2 = Function {
        params: vec![],
        context: Context::new(),
        body: Box::new(Str(CString::new("Hello, hana!").unwrap())),
    };

    // (lambda () x)
    let f3 = Function {
        params: vec![],
        context: Context::new(),
        body: Box::new(Symbol("x".to_string())),
    };

    env.bind_symbol("f1".to_string(), f1);
    env.bind_symbol("f2".to_string(), f2);
    env.bind_symbol("f3".to_string(), f3);

    let mut result = Nil();
    for form in file {
        result = evaluate(form, env);
    }

    println!("{:?}", result);
}
