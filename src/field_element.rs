use std::{ops, process::Output};


#[derive(Debug)]
pub enum ExpressionError {
    DifferentOrderExpression,
    ZeroDivision,
    DifferentCurves,
    InvalidPoint,
    InvalidFieldElement,
}

impl std::error::Error for ExpressionError {}

impl std::fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionError::DifferentOrderExpression => write!(f, "Cannnot add two numbers in different Fields"),
            ExpressionError::ZeroDivision => write!(f, "Cannot divide by zero"),
            ExpressionError::DifferentCurves => write!(f, "Points are not on the same curve"),
            ExpressionError::InvalidPoint => write!(f, "Point is not on the curve"),
            ExpressionError::InvalidFieldElement => write!(f, "Num is not in field range 0 to (Order - 1)"),
        }
    }
}

type Order = u32;

pub trait Modulus {
    fn modulus(self, prime: Order) -> i32;
}

impl Modulus for i32 {
    fn modulus(self, prime: Order) -> Self {
        ((self % prime as i32) + prime as i32) % prime as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    pub num: i32,
    pub prime: Order,
}

impl FieldElement { 
    pub fn new(num: i32, prime: u32) -> Result<FieldElement, ExpressionError> {
        if num >= prime as i32 || num < 0 {
            return Err(ExpressionError::InvalidFieldElement);
        }

        Ok(FieldElement {
            num: num,
            prime: prime,
        })
    }
}

pub trait FieldElementOperation {
    type GeneralOpOutput;

    fn add_op(self, rhs: Self) -> Self::GeneralOpOutput;
    fn sub_op(self, rhs: Self) -> Self::GeneralOpOutput;
    fn mul_op(self, rhs: Self) -> Self::GeneralOpOutput;
    fn div_op(self, rhs: Self) -> Self::GeneralOpOutput;
    fn pow(self, rhs: i32) -> Self;
}

impl FieldElementOperation for FieldElement {
    type GeneralOpOutput = Result<Self, ExpressionError>;

    fn add_op(self, rhs: Self) -> Self::GeneralOpOutput {
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }
        
        Ok(Self { 
            num: (self.num + rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }

    fn sub_op(self, rhs: Self) -> Self::GeneralOpOutput {
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }
        
        Ok(Self { 
            num: (self.num - rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }

    fn mul_op(self, rhs: Self) -> Self::GeneralOpOutput {
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }

        Ok(Self {
            num: (self.num * rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }

    fn div_op(self, rhs: Self) -> Self::GeneralOpOutput {
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }
        if rhs.num == 0 { return Err(ExpressionError::ZeroDivision) }

        Ok(self.mul_op(rhs.pow(-1))?)
    }

    fn pow(self, rhs: i32) -> Self {
        let ex = if rhs < 0 {
            // 指数nが負の場合
            // 指数が正になるまでa^p-1 (= 1) を掛け合わせるので、
            // a^n = a^(n mod p-1)
            rhs.modulus(self.prime - 1)
        } else { rhs };

        let mut num = 1;
        for _ in 0..ex {num = (self.num * num).modulus(self.prime)}
        
        Self {
            num: num,
            prime: self.prime
        }
    }
}

impl ops::Add for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn add(self, rhs: Self) ->  Self::Output{
        self.add_op(rhs)
    }
}

impl ops::Sub for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn sub(self, rhs: Self) ->  Self::Output{
        self.sub_op(rhs)
    }
}

impl ops::Mul for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn mul(self, rhs: Self) -> Self::Output{
        self.mul_op(rhs)
    }
}

impl ops::Div for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_op(rhs)
    }
}


