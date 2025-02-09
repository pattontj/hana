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
