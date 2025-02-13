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
