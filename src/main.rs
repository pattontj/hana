use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;

mod hana;
use hana::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut unparsed_file = fs::read_to_string("tests/bool.hana").expect("cannot read file!");

    let mut funcs = HashMap::new();
    funcs.insert(
        "last",
        "(def last (lambda (lst)
                        (if (= (cdr lst) nil)
                            (car lst)
                            (last (cdr lst)))))",
    );

    let mut builtins = String::new();
    for (_, f) in funcs {
        builtins += f;
        builtins += "\n";
    }

    unparsed_file = builtins + &unparsed_file;

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

    // env.bind_symbol("x".to_string(), Form::Integer(2));
    // env.bind_symbol("y".to_string(), Form::Symbol("x".to_string()));

    // (lambda (x) (+ 2 x))
    // let f1 = Form::Function(Function {
    //     params: vec![Form::Symbol("x".to_string())],
    //     context: Context::new(),
    //     body: Box::new(Form::List(List {
    //         elements: vec![
    //             Box::new(Form::Symbol("+".to_string())),
    //             Box::new(Form::Integer(2)),
    //             Box::new(Form::Symbol("x".to_string())),
    //         ],
    //     })),
    // });

    // (lambda () "Hello, hana!")
    // let f2 = Form::Function(Function {
    //     params: vec![],
    //     context: Context::new(),
    //     body: Box::new(Form::Str(CString::new("Hello, hana!").unwrap())),
    // });

    // (lambda () x)
    // let f3 = Form::Function(Function {
    //     params: vec![],
    //     context: Context::new(),
    //     body: Box::new(Form::Symbol("x".to_string())),
    // });

    // (lambda () (+ 2 x))
    // let f4 = Form::Function(Function {
    //     params: vec![],
    //     context: Context::new(),
    //     body: Box::new(Form::List(List {
    //         elements: vec![
    //             Box::new(Form::Symbol("+".to_string())),
    //             Box::new(Form::Integer(2)),
    //             Box::new(Form::Symbol("x".to_string())),
    //         ],
    //     })),
    // });

    // env.bind_symbol("f1".to_string(), f1);
    // env.bind_symbol("f2".to_string(), f2);
    // env.bind_symbol("f3".to_string(), f3);
    // env.bind_symbol("f4".to_string(), f4);

    let mut result = hana::Form::Nil();
    for form in file {
        result = hana::evaluate(form, env);
    }

    println!("{:?}", result);
}
