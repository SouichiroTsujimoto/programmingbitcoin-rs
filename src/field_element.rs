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

// impl std::error::Error for ExpressionError {}

// impl std::fmt::Display for ExpressionError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Cannnot divide by zero")
//     }
// }

type Order = u32;

trait Modulus {
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
        if num >= prime as i32 || num < 0 || prime < 0 {
            return Err(ExpressionError::InvalidFieldElement);
        }

        Ok(FieldElement {
            num: num,
            prime: prime,
        })
    }
}


impl ops::Add for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn add(self, rhs: Self) ->  Self::Output{
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }
        
        Ok(Self { 
            num: (self.num + rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }
}

impl ops::Sub for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn sub(self, rhs: Self) ->  Self::Output{
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }
        
        Ok(Self { 
            num: (self.num - rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }
}

impl ops::Mul for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn mul(self, rhs: Self) -> Self::Output{
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }

        Ok(Self {
            num: (self.num * rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }
}

impl FieldElement {
    pub fn pow(self, rhs: i32) -> Self{
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

impl ops::Div for FieldElement {
    type Output = Result<Self, ExpressionError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime { return Err(ExpressionError::DifferentOrderExpression) }
        if rhs.num == 0 { return Err(ExpressionError::ZeroDivision) }

        Ok((self * (rhs.pow(-1)))?)
    }
}


