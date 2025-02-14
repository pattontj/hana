use std::any::Any;

use crate::hana::*;

pub fn handle_if(funcall: &List, env: &mut Environment) -> Option<Form> {
    let mut itr = funcall.elements.iter();
    itr.next();

    let conditional = itr.next();

    if let Some(conditional) = conditional {
        let eval = evaluate(*conditional.clone(), env);
        match eval {
            Form::Bool(eval) => {
                if eval == true {
                    let case = itr
                        .next()
                        .expect("Error: could not find form for the true case of if stmt.");
                    return Some(evaluate(*case.clone(), env));
                } else {
                    itr.next();
                    let case = itr
                        .next()
                        .expect("Error: could not find form for the false case of if stmt.");
                    return Some(evaluate(*case.clone(), env));
                }
            }
            _ => {
                println!(
                    "Error: conditional form passed to 'if' must evaluate to a boolean expression."
                );
                return Some(Form::Nil());
            }
        }
    }

    return Some(Form::Nil());
}

/*
    Quote is a special form that takes a form, and tells
    the evaluator not to evaluate it. When a quoted form is
    evaluated, the evaluator will consume the "wrapping" quote,
    returning the form that was initially quoted.
*/

#[allow(dead_code)]
pub fn handle_quote(funcall: &List, _env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() != 2 {
        println!("Error: special form 'quote' takes only a single form as parameter.");
        return Some(Form::Nil());
    }

    let mut itr = funcall.elements.iter();
    itr.next();
    if let Some(quoted) = itr.next() {
        // println!("TEST: {quoted:?}");
        return Some(*quoted.clone());
    }

    Some(Form::Nil())
}

pub fn def_symbol(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function 'def' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let sym = *funcall.elements[1].clone();
    let value = *funcall.elements[2].clone();

    match sym {
        Form::Symbol(sym) => {
            let evaluated = evaluate(value, env);
            env.bind_symbol(sym, evaluated);
        }
        _ => {}
    }

    return Some(Form::Nil());
}

pub fn set_symbol(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function 'def' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let sym = *funcall.elements[1].clone();
    let value = *funcall.elements[2].clone();

    // env.lookup_symbol(sym)
    let symref = match sym.clone() {
        Form::Symbol(sym) => {
            let r = env.lookup_symbol(sym);
            Some(r)
        }
        _ => None,
    };

    if let Some(symref) = symref {
        // let value = symref.unwrap().borrow_mut().clone();

        // *symref.clone().unwrap().borrow_mut() = Form::Symbol("test".to_string());
        *symref.clone().unwrap().borrow_mut() = value;
        // *value = Form::Nil();
        println!("Value: {:?}", symref.unwrap().borrow_mut().clone());
    } else {
        println!(
            "Error: cannot set value of a non-bound symbol ",
            // sym.clone()
        );
    }

    // let value = *funcall.elements[2].clone();

    // match sym {
    //     Form::Symbol(sym) => {
    //         let evaluated = evaluate(value, env);
    //         env.bind_symbol(sym, evaluated);
    //     }
    //     _ => {}
    // }

    return Some(Form::Nil());
}

/*
    Takes a variable number of forms, and evaluates each one from left to right.
    returns the result of the last evaluated form. If no forms are given, it returns
    Nil.
*/
pub fn handle_progn(funcall: &List, env: &mut Environment) -> Option<Form> {
    let mut form = funcall.elements.iter();
    form.next();

    let mut eval = Form::Nil();

    while let Some(form) = form.next() {
        eval = evaluate(*form.clone(), env);
    }

    return Some(eval);
}

/*
    A let special form takes a list of tuples (a list of two elems), and a form to evaluate.
    For each tuple in the list, the first element is expected to be a symbol, and the right value
    is bound to it in a new scope. Once all tuples are bound, the body form is evaluated.
*/
pub fn handle_let(funcall: &List, env: &mut Environment) -> Option<Form> {
    let mut itr = funcall.elements.iter();
    itr.next();

    let tuples = *itr.next().unwrap().clone();
    let body = itr.next().unwrap();

    match tuples {
        Form::List(tuples) => {
            // println!("Tuples list: {tuples:?}");
            for tup in tuples.elements {
                // println!("tup: {tup:?}");

                match *tup {
                    Form::List(tup) => {
                        let sym = tup.elements.first().unwrap();
                        let val = tup.elements.last().unwrap();

                        if let Some(s) = match *sym.clone() {
                            Form::Symbol(s) => Some(s),
                            _ => None,
                        } {
                            // println!("sym: {s:?}, val: {val:?}");
                            env.bind_symbol(s, *val.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {
            println!("Error:");
        }
    }

    let e = evaluate(*body.clone(), env);

    return Some(e);
}

/*
    (each k v list-form body-form)

    Takes two symbol names that represent the key and value in that order, and
    then a list form. For each element in the list in order of appearance,
    the key (default: integer) and value indexed by that key are bound to
    the symbol names given in a new context.

*/
pub fn handle_each(funcall: &List, env: &mut Environment) -> Option<Form> {
    let mut itr = funcall.elements.iter();
    itr.next();

    // if let some(key) = match *itr.next().unwrap().clone() {
    //     form::symbol(_) => {}
    //     _ => {}
    // } {}
    let mut eval: Form = Form::Nil();

    let key = *itr.next().unwrap().clone();
    let value = *itr.next().unwrap().clone();
    let lst = *itr.next().unwrap().clone();
    let body = *itr.next().unwrap().clone();

    match (key.clone(), value.clone(), lst.clone(), body.clone()) {
        (Form::Symbol(k), Form::Symbol(v), lst, _) => {
            println!("{:?}", key);
            println!("{:?}", value);

            let myl: List = match lst.clone() {
                Form::List(lst) => {
                    println!("Not an error");
                    lst
                }
                Form::Symbol(lst) => {
                    println!("Not an error");
                    // let l = lst;
                    let symres = env
                        .lookup_symbol(lst)
                        .unwrap()
                        .as_ref()
                        .borrow_mut()
                        .clone();
                    // println!("{symres:?}");
                    match symres {
                        Form::List(symres) => symres,
                        _ => List { elements: vec![] },
                    }
                }
                _ => {
                    println!("Error?");
                    List { elements: vec![] }
                }
            };

            env.push_new_context();

            for elem in myl.elements.clone().into_iter() {
                println!("v: {v:?}, elem: {elem:?}, body: {body:?}");
                env.bind_symbol(v.clone(), *elem);
                eval = evaluate(body.clone(), env);
                println!("eval :{eval:?}");
            }

            env.pop_context();

            println!("myl: {:?}", myl);
        }
        // (Form::Symbol(key), Form::Symbol(value), Form::List(lst), _) => {
        //     println!("{:?}", key);
        //     println!("{:?}", value);
        //     println!("{:?}", lst);
        // }
        _ => {
            println!("Error? ");
        }
    }

    // return Some(e);
    return Some(eval);
}

fn each_get_kv(key: Form) {}
