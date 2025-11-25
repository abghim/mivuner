
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bin {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Un {
    Sin,
    Cos,
    Ln,
    Exp,
    Neg,
}


#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Var,
    Const(&'static str),
    Num(f32),
    Binary(Bin, Box<Func>, Box<Func>),
    Unary(Un, Box<Func>),
}

use Func::*;

impl Func {
    pub fn sin(x: Func) -> Func {
        Unary(Un::Sin, Box::new(x))
    }

    pub fn cos(x: Func) -> Func {
        Unary(Un::Cos, Box::new(x))
    }

    pub fn ln(x: Func) -> Func {
        Unary(Un::Ln, Box::new(x))
    }

    pub fn exp(x: Func) -> Func {
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
