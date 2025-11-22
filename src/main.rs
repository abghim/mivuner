#![allow(dead_code)]
#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bin {
	Add, Sub, Mul, Div, Pow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Un {
	Sin, Cos, Ln, Exp, Neg
}

#[derive(Debug, Clone, PartialEq)]
enum Func {
	Var,
	Const(&'static str),
	Num(f32),
	Binary(Bin, Box<Func>, Box<Func>),
	Unary(Un, Box<Func>)
}

use std::fmt::Binary;

use Func::*;

impl Func {
	fn sin(x: Func) -> Func{
		Unary(Un::Sin, Box::new(x))
	}

	fn cos(x: Func) -> Func{
		Unary(Un::Cos, Box::new(x))
	}

	fn ln(x: Func) -> Func{
		Unary(Un::Ln, Box::new(x))
	}

	fn exp(x: Func) -> Func{
		Unary(Un::Exp, Box::new(x))
	}
}

impl std::ops::Add for Func {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Binary(Bin::Add, Box::new(self), Box::new(rhs))
	}
}

impl std::ops::Mul for Func {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		Binary(Bin::Mul, Box::new(self), Box::new(rhs))
	}
}

impl std::ops::Sub for Func {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Binary(Bin::Sub, Box::new(self), Box::new(rhs))
	}
}

impl std::ops::Div for Func {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		Binary(Bin::Div, Box::new(self), Box::new(rhs))
	}
}

impl std::ops::Neg for Func {
	type Output = Self;
	fn neg(self) -> Self::Output {
    	Unary(Un::Neg, Box::new(self))
	}
}

impl std::ops::BitXor for Func {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		Binary(Bin::Pow, Box::new(self), Box::new(rhs))
	}
}

fn df(input: &Func) -> Func {
	match input {
		Var => Num(1.0),
		Num(a) => Num(0.0),
		Const(a) => Num(0.0),
		Binary(op, x, y) => {
			match op {
				Bin::Add => df(x)+df(y),
				Bin::Sub => df(x)-df(y),
				Bin::Mul => df(x)*(**y).clone()+df(y)*(**x).clone(),
				Bin::Div => (df(x)*(**y).clone()-df(y)*(**x).clone())/((**y).clone() ^ Num(2.0)),
				Bin::Pow => {
                    let f = (**x).clone();
                    let g = (**y).clone();

                    let f_prime = df(x);
                    let g_prime = df(y);

                    let term = g_prime * Func::ln(f.clone()) + g.clone() * f_prime / f.clone();
                    (f ^ g) * term
                }
			}
		}

		Unary(op, x) => {
			match op {
				Un::Cos => -df(x)*Func::sin((**x).clone()),
				Un::Sin => df(x)*Func::cos((**x).clone()),
				Un::Ln => df(x)/(**x).clone(),
				Un::Exp => df(x)*Func::exp((**x).clone()),
				Un::Neg => -df(x)
			}
		}
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
                Un::Ln  => write!(f, "ln({})",  x),
                Un::Exp => write!(f, "exp({})", x),
                Un::Neg => write!(f, "-({})",   x),
            },

            Func::Binary(op, left, right) => match op {
                Bin::Add => write!(f, "({} + {})", left, right),
                Bin::Sub => write!(f, "({} - {})", left, right),
                Bin::Mul => write!(f, "({} * {})", left, right),
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
		},

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

				_ => new.clone()
			}
		}

		_ => (*input).clone()
	}

}

fn main() {
	let test: Func = (Num(2.0)*(Var^Num(2.0))-Func::sin(Func::exp(Var))-(Var^Num(0.5))/Func::ln(Var))*(Num(3.0)*Var-Num(1.0))/(Var^Num(4.0));
	let test2: Func = Num(2.0)*(Var^Num(2.0));
	println!("{}", simplify(&df(&test)));
	println!("{}", simplify(&df(&test2)));
}
