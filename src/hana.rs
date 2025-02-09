// use std::borrow::{Borrow, BorrowMut};
// use std::borrow::{Borrow, BorrowMut};
// use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
// use std::env;
use std::ffi::CString;
// use std::fs;
use std::iter::zip;
// use std::ops::{Deref, DerefMut};
use std::rc::Rc;

// use self::Form::*;

pub mod builtin;
pub mod special;
use builtin::builtin_function;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "hana.pest"]
pub struct HanaParser;

pub type Integer = i32;
pub type Real = f64;
pub type Str = CString;
pub type Symbol = String;

/*
    In Hana, a form is any valid data that can be evaluated by the evaluator.
*/
#[derive(Debug, Clone, PartialEq)]
pub enum Form {
    Integer(Integer),
    Real(Real),
    Str(Str),
    Bool(bool),
    Symbol(Symbol),
    List(List),
    Function(Function),
    Nil(),
}

impl Default for Form {
    fn default() -> Self {
        Form::Nil()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub elements: Vec<Box<Form>>,
}

impl List {
    pub fn get_symbols_from_list(&self) -> Vec<String> {
        let mut itr = self.elements.iter();

        let mut ret = vec![];

        while let Some(itr) = itr.next() {
            match *itr.clone() {
                Form::Symbol(itr) => {
                    ret.push(itr);
                }
                Form::List(itr) => {
                    let mut l = itr.get_symbols_from_list();
                    ret.append(&mut l);
                }
                _ => {}
            }
        }
        ret
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub params: Vec<Form>,
    pub context: Context,
    pub body: Box<Form>,
}

impl Function {
    /*
        Takes a list of args from where it's called and the current env,
        and attempts to bind each param symbol to it's positionally equivalent
        arg within the current context.
    */
    pub fn bind_params(&mut self, args: Vec<Box<Form>>, _env: &mut Environment) {
        match *self.body {
            Form::List(ref list) => {
                for f in list.clone().elements {
                    println!("body element: {:?}", f);
                }
            }
            _ => {
                println!("aaaaa");
            }
        }

        for (param, arg) in zip(&self.params, args) {
            match param {
                Form::Symbol(param) => self.context.bind_symbol(param.clone(), *arg),
                _ => {
                    println!("Error: formal parameter {param:?} is not a symbol");
                }
            }
        }
    }

    /*
        Binds a symbol to a function's internal context by fetching it's Rc-Refcell
        via an env lookup.
    */
    pub fn bind_to_context(&mut self, env: &mut Environment, sym: String) {
        let formref = env.bindings.last().unwrap().lookup_symbol(sym.clone());
        if let Some(formref) = formref {
            self.context.insert(sym, formref.clone());
        } else {
            println!("Error: could not find symbol {sym:?} in the environment.");
        }
    }

    /*
        Takes the current environment at time of creation and attempts to bind each explicitly
        mentioned symbol to the fn's internal context, effectively closing over the env.
    */
    pub fn close_over_env(&mut self, env: &mut Environment) {
        let body = self.body.clone();

        // Fish through the body for any and all symbols, bind those to the fn's context

        match *body {
            // if it's just a symbol, lookup and then manually insert into the function's context
            Form::Symbol(bd) => {
                self.bind_to_context(env, bd);
                // return Some(*body.clone());
            }

            // if it's a list, parse said list for symbols and do the same as above.
            // If the symbol in the fn body matches a param, then we don't bind it
            // as lexical scoping would indicate that it is not a value being closed over.
            Form::List(body) => {
                let mut itr = body.elements.iter();
                // itr.next();
                while let Some(itr) = itr.next() {
                    let b = *itr.clone();
                    match b {
                        Form::Symbol(b) => {
                            // HACK: checking against a const arr of built-in names.
                            if !self.params.contains(itr)
                                && !builtin::BUILTIN_SYMBOLS.contains(&&*b.clone().into_boxed_str())
                            {
                                self.bind_to_context(env, b);
                            }
                        }
                        Form::List(b) => {
                            // fetches all symbols in a list as strings, including from sub-lists
                            let symbols = b.get_symbols_from_list();
                            for s in symbols {
                                println!("symbol found: {s:?}");
                                self.bind_to_context(env, s)
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                println!("Error: something");
            }
        }
    }
}

// Parses raw text and returns the code as a valid list of forms to be evaluated,
// or an error if a grammar error is present.
pub fn parse(source: &str) -> Result<Vec<Form>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = HanaParser::parse(Rule::program, source)?;

    for pair in pairs {
        match pair.as_rule() {
            Rule::form => {
                ast.push(build_ast_from_form(pair));
            }
            Rule::quoted_form => {
                // println!("Quoted form identified");
                // println!("from parse, pair?: {pair:?}");
                ast.push(build_ast_from_quoted_form(pair));
            }
            _ => {}
        }
    }

    Ok(ast)
}

/*
    Helper function for parse, builds a valid AST-representation Form
    from a quoted form grammar rule. Effectively replaces any instance of
    ->    'form
    with
    ->    (quote form)
*/
pub fn build_ast_from_quoted_form(pair: pest::iterators::Pair<Rule>) -> Form {
    let mut quoted = List { elements: vec![] };
    quoted
        .elements
        .push(Box::new(Form::Symbol("quote".to_string())));
    // println!("PAIR: {pair:?}");
    let f = build_ast_from_form(pair.into_inner().next().unwrap());
    // println!("quoted form ast: {f:?}");
    quoted.elements.push(Box::new(f));

    Form::List(quoted)
}

/*
    The 'Reader' for Hana. Takes a pest pair generated by the parser, and
    constructs a valid form before returning it.
*/
pub fn build_ast_from_form(pair: pest::iterators::Pair<Rule>) -> Form {
    match pair.as_rule() {
        Rule::integer => {
            // println!("integer pair? {:?}", pair);
            let i: i32 = pair.as_str().parse().unwrap();
            Form::Integer(i)
        }
        Rule::real => {
            let r: f64 = pair.as_str().parse().unwrap();
            Form::Real(r)
        }
        Rule::string => {
            let str = &pair.as_str();
            // println!("{}", str);
            let str = &str[1..str.len() - 1];
            // println!("{}", str);
            // should be: \" -> "
            let str = str.replace("\\\"", "\"");

            Form::Str(CString::new(&str[..]).unwrap())
        }

        Rule::bool => {
            let b: bool = pair.as_str().parse().unwrap();
            Form::Bool(b)
        }
        Rule::symbol => Form::Symbol(String::from(pair.as_str())),

        Rule::list => {
            let mut pair = pair.into_inner();

            let mut elements = Vec::new();

            while let Some(p) = pair.next() {
                // println!("{:?}", p);
                elements.push(Box::new(build_ast_from_form(p)));
            }

            Form::List(List { elements })
        }

        Rule::quoted_form => build_ast_from_quoted_form(pair.into_inner().next().unwrap()),

        Rule::form => build_ast_from_form(pair.into_inner().next().unwrap()),
        _ => {
            // println!("Hitting edge case in build_ast_from_form");
            Form::Nil()
        }
    }
}

/*
    In Hana, a context is a HashMap that binds symbols to valid forms.
    Each context only holds the set of bindings made in it's respective lexical scope.
*/
pub type Context = HashMap<Symbol, Rc<RefCell<Form>>>;

/*
    Using an extension trait to make it a little prettier to use Contexts
*/
pub trait ContextExt {
    fn bind_symbol(&mut self, symbol: Symbol, value: Form);
    fn lookup_symbol(&self, symbol: Symbol) -> Option<&Rc<RefCell<Form>>>;
}

impl ContextExt for Context {
    fn bind_symbol(&mut self, symbol: Symbol, value: Form) {
        match value {
            Form::Symbol(value) => {
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

    fn lookup_symbol(&self, symbol: Symbol) -> Option<&Rc<RefCell<Form>>> {
        self.get(&symbol)
    }
}

/*
    An environment is a global struct that holds contextual information required to evaluate
    forms in Hana (and Lisp more generally). In Hana specifically, the environment holds
    a lexical stack of 'Context's, each of which are a HashMap that maps symbols to
    a valid form held in a Rc<RefCell<Form>>.
*/
pub struct Environment {
    pub bindings: Vec<Context>,
}

impl Environment {
    // Attempts to bind a valid form to a symbol in the topmost context in the context-stack.
    pub fn bind_symbol(&mut self, symbol: Symbol, value: Form) {
        let len = self.bindings.len();
        let ctx = self.bindings.get_mut(len - 1).unwrap();

        ctx.bind_symbol(symbol, value);
    }
    // Attempts to find a form bound to the given symbol (passed as string), starting
    // from the top of the context stack, working its way down.
    // returns None if a binding is not found, otherwise returns the binding.
    pub fn lookup_symbol(&self, symbol: Symbol) -> Option<Rc<RefCell<Form>>> {
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

    // Pushes a new context to the top of the context-stack.
    pub fn push_context(&mut self) {
        self.bindings.push(Context::new());
    }

    // Pops the topmost context from the context-stack.
    #[allow(dead_code)]
    pub fn pop_context(&mut self) {
        self.bindings.pop();
    }
}

/*
    The 'Evaluator' for Hana. Takes a valid form, and the environment in which
    the form is to be evaluated in, and returns the result of the evaluation
    as a valid form.

    Some forms in Hana (and Lisps generally) are considered to be 'self-evaluating'
    when the result of evaluating a form is the form itself, I.E. evaluate(form) = form.
    The evaluator will recurse on any non-self-evaluating form until it reaches a
    self-evaluating form.

    This means that the resulting evaluation of any valid Hana program will produce
    a self-evaluating form as a result.
*/
pub fn evaluate(form: Form, env: &mut Environment) -> Form {
    match form {
        Form::Integer(_) => form,

        Form::Real(_) => form,

        Form::Str(_) => form,

        Form::Bool(_) => form,

        Form::Symbol(form) => {
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

            Form::Nil()
        }

        Form::List(list) => {
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
            // let elements = &list.elements;

            // if it's an empty list, return nil
            if list.elements.len() == 0 {
                return Form::Nil();
            }

            let first = *list.elements[0].clone();

            // fetch the first element, check if it's a symbol,
            // if it is then bind the args to the function's parameters
            match first {
                Form::Symbol(first) => {
                    if let Some(builtin) = builtin_function(&first, &list, env) {
                        println!("Built-in function found: {first:?}");
                        return builtin;
                    }
                    if let Some(lookup) = env.lookup_symbol(first) {
                        println!("Lookup: {:?}", lookup);

                        let fun = lookup.as_ref().borrow_mut().clone();

                        /*
                        IDEA: Have a 'built-in function' struct of some kind that
                            can be used to differentiate from a regular function.
                            That way it can be matched in the following block and
                            be handled appropriately.
                        */

                        // if the symbol is a function, treat it as a function call
                        match fun {
                            Form::Function(mut fun) => {
                                println!("[DEBUG] Valid function form");

                                println!("\tparams: {:?}", fun.params);
                                println!("\tcontext: {:?}", fun.context);
                                println!("\tbody: {:?}", fun.body);
                                println!("\n");

                                // create a new context, and for each formal parameter, bind the args passed
                                // in the form being evaluated

                                env.push_context();

                                /*
                                    PROBLEM:
                                    Currently the Function data type claims to hold onto a context object.
                                    This poses a problem because all contexts are owned by the environment currently,
                                    and trying to do any kind of ref-counting on contexts will end up being a mental pain.

                                    SOLUTION:
                                    The function will own it's own context. Inside that context will be:
                                    1. Bound parameters
                                    2. Closed-over values (I.E. cloned Rc-RefCells of only the explicitly referenced symbols)

                                    When a function is called and has to resolve a symbol to a form, it will first query its own
                                    context, and then after will query the environment.

                                    To achieve this functionality, the evaluator will examine the body of the function and build
                                    a list of any symbol(s) that do not match the name of the function's formal parameters.
                                    It will then perform a lookup on that outer symbol, and bind that symbol's associated
                                    value within its own personal context.

                                    When a symbol is bound to a value in Hana, that value is first evaluated; when that value
                                    is a symbol, this means that the newly bound symbol is directly bound to a clone of
                                    the associated Rc-Refcell.

                                    This means that the referenced symbol-value bindings are closed over, and since the binding holds a
                                    reference-counted smart pointer, the original context it was defined in may be cleaned up while
                                    the binding and value are maintained.


                                */

                                // println!("\t=Matching param to args=");
                                println!("\t=Testing bind_params=");

                                if let Some((_, rest)) = list.elements.split_first() {
                                    fun.bind_params(rest.to_vec(), env);
                                }

                                println!("[DEBUG] Valid function form");

                                println!("\tparams: {:?}", fun.params);
                                println!("\tcontext: {:?}", fun.context);
                                println!("\tbody: {:?}", fun.body);
                                println!("\n");

                                // if let Some((_, rest)) = elements.as_slice().split_first() {
                                //     for (param, arg) in zip(fun.params, rest) {
                                //         println!("param={param:?}, arg={arg:?}");
                                //         match param {
                                //             Symbol(param) => {
                                //                 env.bind_symbol(param, *arg.clone());
                                //             }

                                //             _ => {
                                //                 println!("Error: formal parameter {param:?} is not a symbol");
                                //             }
                                //         }
                                //     }
                                // }

                                //
                                // match *fun.body {
                                //     Form::List { ref elements } => {
                                //         println!("test");
                                //         for e in elements.clone() {
                                //             println!("list elem in body of function: {e:?}");
                                //             // if !fun.params.contains(&*e) {
                                //             //     println!("'{e:?}' not in param list");
                                //             // }
                                //         }
                                //     }
                                //     Form::Function { .. } => {
                                //         println!("aaa");
                                //     }
                                //     _ => {
                                //         println!("aaa");
                                //     }
                                // }

                                // for param in params {
                                //     println!("parameter: {param:?}");
                                //     println!("parameter: {param:?}");
                                // }

                                return evaluate(*fun.body, env);
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

            for elem in list.elements {
                let res = evaluate(*elem, env);
                println!("evaluated: {res:?}");
            }
            // println!("elements: {:?}", elements);
            Form::Nil()
        }

        _ => Form::Nil(),
    }
}
