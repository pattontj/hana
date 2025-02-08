use crate::hana::*;

// Takes refs to a symbol and the current environment, and compares the symbol
// against a set of built-in functions
pub fn builtin_function(symbol: &Symbol, funcall: &List, env: &mut Environment) -> Option<Form> {
    match symbol.as_str() {
        "+" => handle_add(funcall, env),
        "-" => handle_sub(funcall, env),
        "*" => handle_mul(funcall, env),
        "/" => handle_div(funcall, env),
        _ => {
            return None;
        }
    }
}

fn handle_add(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '+' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let mut itr = funcall.elements.iter();

    // skip the function name
    itr.next();

    let mut sum: Real = 0.0;

    while let Some(itr) = itr.next() {
        println!("individual elem: {itr:?}");

        let evaluated = evaluate(*itr.clone(), env);

        match evaluated {
            Form::Integer(evaluated) => {
                sum += evaluated as Real;
            }

            Form::Real(evaluated) => {
                sum += evaluated;
            }
            _ => {
                println!("Error: {evaluated:?} is not a a number.");
                return Some(Form::Nil());
            }
        }

        println!("individual evaluated elem: {evaluated:?}");
    }

    return Some(Form::Real(sum));
}

// fn handle_add(funcall: &List, env: &mut Environment) -> Option<Form> {}

fn handle_sub(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '+' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let mut itr = funcall.elements.iter();

    // skip the function name
    itr.next();

    let mut sub: Real = 0.0;

    if let Some(itr) = itr.next() {
        let evaluated = evaluate(*itr.clone(), env);
        match evaluated {
            Form::Real(evaluated) => sub = evaluated,

            Form::Integer(evaluated) => sub = evaluated as Real,
            _ => {
                println!("Error: argument {evaluated:?} is not a number.");
                return Some(Form::Nil());
            }
        }
    }

    while let Some(itr) = itr.next() {
        println!("individual elem: {itr:?}");

        let evaluated = evaluate(*itr.clone(), env);

        match evaluated {
            Form::Integer(evaluated) => {
                sub -= evaluated as Real;
            }

            Form::Real(evaluated) => {
                sub -= evaluated;
            }
            _ => {
                println!("Error: {evaluated:?} is not a a number.");
                return Some(Form::Nil());
            }
        }

        println!("individual evaluated elem: {evaluated:?}");
    }

    return Some(Form::Real(sub));
}

fn handle_mul(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let mut itr = funcall.elements.iter();

    // skip the function name
    itr.next();

    let mut product: Real = 0.0;

    if let Some(itr) = itr.next() {
        let evaluated = evaluate(*itr.clone(), env);
        match evaluated {
            Form::Real(evaluated) => product = evaluated,

            Form::Integer(evaluated) => product = evaluated as Real,
            _ => {
                println!("Error: argument {evaluated:?} is not a number.");
                return Some(Form::Nil());
            }
        }
    }

    while let Some(itr) = itr.next() {
        println!("individual elem: {itr:?}");

        let evaluated = evaluate(*itr.clone(), env);

        match evaluated {
            Form::Integer(evaluated) => {
                product *= evaluated as Real;
            }

            Form::Real(evaluated) => {
                product *= evaluated;
            }
            _ => {
                println!("Error: {evaluated:?} is not a a number.");
                return Some(Form::Nil());
            }
        }

        println!("individual evaluated elem: {evaluated:?}");
    }

    return Some(Form::Real(product));
}

fn handle_div(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let mut itr = funcall.elements.iter();

    // skip the function name
    itr.next();

    let mut quot: Real = 0.0;

    if let Some(itr) = itr.next() {
        let evaluated = evaluate(*itr.clone(), env);
        match evaluated {
            Form::Real(evaluated) => quot = evaluated,

            Form::Integer(evaluated) => quot = evaluated as Real,
            _ => {
                println!("Error: argument {evaluated:?} is not a number.");
                return Some(Form::Nil());
            }
        }
    }

    while let Some(itr) = itr.next() {
        println!("individual elem: {itr:?}");

        let evaluated = evaluate(*itr.clone(), env);

        match evaluated {
            Form::Integer(evaluated) => {
                if evaluated == 0 {
                    println!("Error: cannot divide by zero");
                    return Some(Form::Nil());
                }
                quot /= evaluated as Real;
            }

            Form::Real(evaluated) => {
                if evaluated == 0.0 {
                    println!("Error: cannot divide by zero");
                    return Some(Form::Nil());
                }
                quot /= evaluated;
            }
            _ => {
                println!("Error: {evaluated:?} is not a a number.");
                return Some(Form::Nil());
            }
        }

        println!("individual evaluated elem: {evaluated:?}");
    }

    return Some(Form::Real(quot));
}

// fn handle_add(funcall: &List, env: &mut Environment) -> Option<Form> {
// }
