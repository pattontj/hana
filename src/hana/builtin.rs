use crate::hana::special::*;
use crate::hana::*;

pub const BUILTIN_SYMBOLS: [&str; 12] = [
    "lambda", "lambda", "if", "+", "-", "*", "/", "<", "<=", ">", ">=", "=",
];

// Takes refs to a symbol and the current environment, and compares the symbol
// against a set of built-in functions
pub fn builtin_function(symbol: &Symbol, funcall: &List, env: &mut Environment) -> Option<Form> {
    match symbol.as_str() {
        "quote" => handle_quote(funcall, env),
        "lambda" => make_lambda(funcall, env),
        "if" => handle_if(funcall, env),
        "+" => handle_add(funcall, env),
        "-" => handle_sub(funcall, env),
        "*" => handle_mul(funcall, env),
        "/" => handle_div(funcall, env),
        "<" => handle_lt(funcall, env),
        "<=" => handle_lte(funcall, env),
        ">" => handle_gt(funcall, env),
        ">=" => handle_gte(funcall, env),
        "=" => handle_eq(funcall, env),
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
        // println!("individual elem: {itr:?}");

        let evaluated = evaluate(*itr.clone(), env);

        // println!("Evaluated?: {evaluated:?}");
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

        // println!("individual evaluated elem: {evaluated:?}");
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

fn handle_lt(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }
    let mut itr = funcall.elements.iter();
    itr.next();

    let mut lhs: f64 = 0.0;
    let mut rhs: f64 = 0.0;

    if let Some(l) = itr.next() {
        let eval = evaluate(*l.clone(), env);
        match eval {
            Form::Integer(eval) => lhs = eval as f64,
            Form::Real(eval) => lhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }
    if let Some(r) = itr.next() {
        let eval = evaluate(*r.clone(), env);
        match eval {
            Form::Integer(eval) => rhs = eval as f64,
            Form::Real(eval) => rhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }

    Some(Form::Bool(lhs < rhs))
}
fn handle_lte(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }
    let mut itr = funcall.elements.iter();
    itr.next();

    let mut lhs: f64 = 0.0;
    let mut rhs: f64 = 0.0;

    if let Some(l) = itr.next() {
        let eval = evaluate(*l.clone(), env);
        match eval {
            Form::Integer(eval) => lhs = eval as f64,
            Form::Real(eval) => lhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }
    if let Some(r) = itr.next() {
        let eval = evaluate(*r.clone(), env);
        match eval {
            Form::Integer(eval) => rhs = eval as f64,
            Form::Real(eval) => rhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }

    Some(Form::Bool(lhs <= rhs))
}

fn handle_gt(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }
    let mut itr = funcall.elements.iter();
    itr.next();

    let mut lhs: f64 = 0.0;
    let mut rhs: f64 = 0.0;

    if let Some(l) = itr.next() {
        let eval = evaluate(*l.clone(), env);
        match eval {
            Form::Integer(eval) => lhs = eval as f64,
            Form::Real(eval) => lhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }
    if let Some(r) = itr.next() {
        let eval = evaluate(*r.clone(), env);
        match eval {
            Form::Integer(eval) => rhs = eval as f64,
            Form::Real(eval) => rhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }

    Some(Form::Bool(lhs > rhs))
}
fn handle_gte(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }
    let mut itr = funcall.elements.iter();
    itr.next();

    let mut lhs: f64 = 0.0;
    let mut rhs: f64 = 0.0;

    if let Some(l) = itr.next() {
        let eval = evaluate(*l.clone(), env);
        match eval {
            Form::Integer(eval) => lhs = eval as f64,
            Form::Real(eval) => lhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }
    if let Some(r) = itr.next() {
        let eval = evaluate(*r.clone(), env);
        match eval {
            Form::Integer(eval) => rhs = eval as f64,
            Form::Real(eval) => rhs = eval,
            _ => {
                println!("Error: Cannot compare inequality for non-numerical types.");
                return Some(Form::Nil());
            }
        }
    }

    Some(Form::Bool(lhs >= rhs))
}

/*
    Compares the value of two forms. If the form is a symbol,
    the value that's bound to it will be compared.
*/
fn handle_eq(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function '*' takes >= 2 parameters");
        return Some(Form::Nil());
    }
    let mut itr = funcall.elements.iter();
    itr.next();
    let mut lhs = *itr
        .next()
        .expect("Error: cannot retrieve second argument from function call to eq")
        .clone();
    let mut rhs = *itr
        .next()
        .expect("Error: cannot retrieve third argument from function call to eq")
        .clone();

    lhs = evaluate(lhs, env);
    rhs = evaluate(rhs, env);

    Some(Form::Bool(lhs == rhs))

    // Some(Form::Nil())
}

/*
    Creates a new function via the (lambda) function call, closing over
    any referenced symbols in the environment at time of creation.
*/
fn make_lambda(funcall: &List, env: &mut Environment) -> Option<Form> {
    if funcall.elements.len() < 3 {
        println!("Error: function 'lambda' takes >= 2 parameters");
        return Some(Form::Nil());
    }

    let mut fun: Function = Function {
        params: vec![],
        context: HashMap::new(),
        body: Box::new(Form::Nil()),
    };

    // Skip the function name
    let mut itr = funcall.elements.iter();
    itr.next();

    // copy the parameter form
    if let Some(params) = itr.next() {
        match *params.clone() {
            Form::List(params) => {
                // println!("params?: {params:?}");
                for elem in params.elements {
                    fun.params.push(*elem);
                }
            }
            _ => {
                println!("Error: ");
            }
        }
    }

    // Copy the body form
    if let Some(body) = itr.next() {
        fun.body = body.clone();
    }

    // fishes through the body of the function for refs to symbols in
    // lexical scopes outside it's own, and creates ref. counted smart pointer
    // clones that get stored in it's own context, effectively closing over that binding.
    fun.close_over_env(env);

    return Some(Form::Function(fun));
}

// fn make_lambda(funcall: &List, env: &mut Environment) -> Option<Form> {
//     if funcall.elements.len() < 3 {
//         println!("Error: function 'lambda' takes >= 2 parameters");
//         return Some(Form::Nil());
//     }

//     // let mut fun: Function;
//     let mut fun: Function = Function {
//         params: vec![],
//         context: HashMap::new(),
//         body: Box::new(Form::Nil()),
//     };

//     let mut itr = funcall.elements.iter();
//     itr.next();

//     if let Some(params) = itr.next() {
//         match *params.clone() {
//             Form::List(params) => {
//                 println!("params?: {params:?}");
//                 for elem in params.elements {
//                     fun.params.push(*elem);
//                 }
//             }
//             _ => {
//                 println!("Error: ");
//             }
//         }
//     }

//     // Fish through the body for any and all symbols, bind those to the fn's context
//     if let Some(body) = itr.next() {
//         fun.body = body.clone();

//         let bd = body.clone();
//         match *bd {
//             // if it's just a symbol, lookup and then manually insert into the function's context
//             Form::Symbol(bd) => {
//                 fun.bind_to_context(env, bd);
//                 // return Some(*body.clone());
//             }

//             // if it's a list, parse said list for symbols and do the same as above.
//             // If the symbol in the fn body matches a param, then we don't bind it
//             // as lexical scoping would indicate that it is not a value being closed over.

//             // NOTE: This will not parse an inner list. This functionality needs to be broken
//             // off into a function that can be recursively called to fix this.
//             Form::List(body) => {
//                 let mut itr = body.elements.iter();
//                 // itr.next();
//                 while let Some(itr) = itr.next() {
//                     println!("TESDT>");
//                     let b = *itr.clone();
//                     match b {
//                         Form::Symbol(b) => {
//                             if !fun.params.contains(itr)
//                                 && !BUILTIN_SYMBOLS.contains(&&*b.clone().into_boxed_str())
//                             {
//                                 fun.bind_to_context(env, b);
//                             }
//                         }
//                         _ => {}
//                     }
//                     // println!("ELEM: {b:?}");
//                 }

//                 // for elem in body.elements {
//                 //     println!("ELEM: {elem:?}");
//                 // }
//             }
//             _ => {
//                 println!("Error: ");
//             }
//         }
//     }

//     return Some(Form::Function(fun));
// }

// fn handle_add(funcall: &List, env: &mut Environment) -> Option<Form> {
// }
