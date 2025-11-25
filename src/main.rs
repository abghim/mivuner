#![allow(dead_code)]
#![allow(unused)]

mod types;
use types::*;
use types::Func::*;
// use lrlex::lrlex_mod;
// use lrpar::lrpar_mod;

// // Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the module name
// // will be `calc_l` (i.e. the file name, minus any extensions, with a suffix of `_l`).
// lrlex_mod!("calc.l");
// // Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the module name
// // will be `calc_y` (i.e. the file name, minus any extensions, with a suffix of `_y`).
// lrpar_mod!("calc.y");



fn df(input: &Func) -> Func {
    match input {
        Var => Num(1.0),
        Num(a) => Num(0.0),
        Const(a) => Num(0.0),
        Binary(op, x, y) => match op {
            Bin::Add => df(x) + df(y),
            Bin::Sub => df(x) - df(y),
            Bin::Mul => df(x) * (**y).clone() + df(y) * (**x).clone(),
            Bin::Div => {
                (df(x) * (**y).clone() - df(y) * (**x).clone()) / ((**y).clone() ^ Num(2.0))
            }
            Bin::Pow => {
                if let Num(k) = (**y).clone() {
                    Num(k) * ((**x).clone() ^ Num(k - 1.0)) * df(x)
                } else {
                    let f = (**x).clone();
                    let g = (**y).clone();

                    let f_prime = df(x);
                    let g_prime = df(y);

                    let term = g_prime * Func::ln(f.clone()) + g.clone() * f_prime / f.clone();
                    (f ^ g) * term
                }
            }
        },

        Unary(op, x) => match op {
            Un::Cos => -df(x) * Func::sin((**x).clone()),
            Un::Sin => df(x) * Func::cos((**x).clone()),
            Un::Ln => df(x) / (**x).clone(),
            Un::Exp => df(x) * Func::exp((**x).clone()),
            Un::Neg => -df(x),
        },
    }
}

impl std::fmt::Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Func::Var => write!(f, "x"),
            Func::Const(name) => write!(f, "{}", name),
            Func::Num(n) => write!(f, "{}", n),

            Func::Unary(op, x) => match op {
                Un::Sin => write!(f, "sin({})", x),
                Un::Cos => write!(f, "cos({})", x),
                Un::Ln => write!(f, "ln({})", x),
                Un::Exp => write!(f, "exp({})", x),
                Un::Neg => write!(f, "-({})", x),
            },

            Func::Binary(op, left, right) => match op {
                Bin::Add => write!(f, "({} + {})", left, right),
                Bin::Sub => write!(f, "({} - {})", left, right),
                Bin::Mul => {
                    if let Num(n) = (**left).clone() {
                        if let Num(k) = (**right).clone() {
                            write!(f, "{}*{}", n, k)
                        } else {
                            write!(f, "{}{}", n, right)
                        }
                    } else {
                        write!(f, "({} * {})", left, right)
                    }
                }
                Bin::Div => write!(f, "({} / {})", left, right),
                Bin::Pow => write!(f, "({} ^ {})", left, right),
            },
        }
    }
}

fn simplify(input: &Func) -> Func {
    match input {
        Binary(op, rx, ry) => {
            let x = simplify(rx);
            let y = simplify(ry);

            let new = Binary((*op).clone(), Box::new(x.clone()), Box::new(y.clone()));

            match op {
                Bin::Add => {
                    if (x).clone() == Num(0.0) {
                        (y).clone()
                    } else if (y).clone() == Num(0.0) {
                        (x).clone()
                    } else {
                        new.clone()
                    }
                }
                Bin::Sub => {
                    if (x).clone() == Num(0.0) {
                        -((y).clone())
                    } else if (y).clone() == Num(0.0) {
                        (x).clone()
                    } else {
                        new.clone()
                    }
                }
                Bin::Mul => {
                    if (x).clone() == Num(1.0) {
                        (y).clone()
                    } else if (y).clone() == Num(1.0) {
                        (x).clone()
                    } else if (y).clone() == Num(1.0) || (x).clone() == Num(0.0) {
                        Num(0.0)
                    } else {
                        new.clone()
                    }
                }

                Bin::Div => {
                    if (y).clone() == Num(0.0) {
                        panic!("Division by zero");
                    } else if (x).clone() == Num(0.0) {
                        Num(0.0)
                    } else if (y).clone() == Num(1.0) {
                        (x).clone()
                    } else {
                        new.clone()
                    }
                }

                Bin::Pow => {
                    if (y).clone() == Num(0.0) {
                        Num(1.0)
                    } else if (y).clone() == Num(1.0) {
                        (x).clone()
                    } else if (x).clone() == Num(0.0) {
                        Num(0.0)
                    } else {
                        new.clone()
                    }
                }
            }
        }

        Unary(op, rx) => {
            let x = &simplify(rx);
            let new = Unary((*op).clone(), Box::new(x.clone()));

            match op {
                Un::Ln => {
                    if let Num(k) = x {
                        if *k <= 0.0 {
                            panic!("ln undefined");
                        } else {
                            new.clone()
                        }
                    } else {
                        new.clone()
                    }
                }
                Un::Neg => {
                    if let Unary(Un::Neg, k) = (x).clone() {
                        (*k).clone()
                    } else {
                        new.clone()
                    }
                }

                _ => new.clone(),
            }
        }

        _ => (*input).clone(),
    }
}

fn main() {
    let test: Func = (Num(2.0) * (Var ^ Num(2.0))
        - Func::sin(Func::exp(Var))
        - (Var ^ Num(0.5)) / Func::ln(Var))
        * (Num(3.0) * Var - Num(1.0))
        / (Var ^ Num(4.0));
    let test2: Func = Num(2.0) * (Var ^ Num(2.0));
    let test3: Func = Num(2.0) * Func::sin(Num(2.0) * Var);


    println!("{}", simplify(&df(&test3)))

    // use std::io::{self, BufRead, Write};
    // // Get the `LexerDef` for the `calc` language.
    // let lexerdef = calc_l::lexerdef();
    // let stdin = io::stdin();
    // loop {
    //     print!(">>> ");
    //     io::stdout().flush().ok();
    //     match stdin.lock().lines().next() {
    //         Some(Ok(ref l)) => {
    //             if l.trim().is_empty() {
    //                 continue;
    //             }
    //             // Now we create a lexer with the `lexer` method with which we can lex an input.
    //             let lexer = lexerdef.lexer(l);
    //             // Pass the lexer to the parser and lex and parse the input.
    //             let (res, errs) = calc_y::parse(&lexer);
    //             for e in errs {
    //                 println!("{}", e.pp(&lexer, &calc_y::token_epp));
    //             }
    //             match res {
    //                 Some(Ok(r)) => println!("Result: {}", r),
    //                 _ => eprintln!("Unable to evaluate expression.")
    //             }
    //         }
    //         _ => break
    //     }
    // }
}
