use std::{ops, process::Output};

use num_traits::Zero;


#[derive(Debug)]
struct InvalidFieldElement;

impl std::error::Error for InvalidFieldElement {}

impl std::fmt::Display for InvalidFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Num is not in field range 0 to (Order - 1)")
    }
}

#[derive(Debug)]
enum ExpressionError {
    DifferentOrderExpression,
    ZeroDivision,
}

impl std::error::Error for ExpressionError {}

impl std::fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionError::DifferentOrderExpression => write!(f, "Cannnot add two numbers in different Fields"),
            ExpressionError::ZeroDivision => write!(f, "Cannot divide by zero"),
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
struct FieldElement {
    num: i32,
    prime: Order,
}

impl FieldElement { 
    pub fn new(num: i32, prime: u32) -> Result<FieldElement, InvalidFieldElement> {
        if num >= prime as i32 || num < 0 || prime < 0 {
            return Err(InvalidFieldElement);
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
    fn pow(self, rhs: i32) -> Self{
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




fn main() {
    
    let a = FieldElement::new(2,13).unwrap();
    let b = FieldElement::new(4,13).unwrap();
    
    println!("{:#?}", b.pow(2) * a.pow(-4));
    
    println!("Hello, world!");
}




#[cfg(test)]
mod tests {
    use core::panic;

    use crate::{FieldElement, InvalidFieldElement};

    #[test]
    fn field_element_eq_and_ne() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(10, 29)?;

        assert!(a == a);

        let c = FieldElement::new(9, 29)?;
        let d = FieldElement::new(10, 31)?;

        assert!(a != c);
        assert!(a != d);
        

        Ok(())
    }

    #[test]
    #[should_panic]
    fn field_element_invalid_num() {
        match FieldElement::new(30, 29) {
            Ok(_) => println!("why?"),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic]
    fn field_element_minus_num() {
        match FieldElement::new(-30, 31) {
            Ok(_) => println!("why?"),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn field_element_add_and_sub() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(7,13)?;
        let b = FieldElement::new(12,13)?;
        let c = FieldElement::new(6,13)?;

        assert_eq!((a+b)?, c);

        assert_eq!((c-b)?, a);

        Ok(())
    }

    #[test]
    #[should_panic]
    fn field_element_add_with_different_order() {
        let a = match FieldElement::new(7,13) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        let b = match FieldElement::new(12,17) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        
        match a + b {
            Ok(_) => println!("why?"),
            Err(_) => panic!("Invalid expression"),
        }
    }

    #[test]
    fn field_element_mul_and_pow() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(2,13)?;
        let b = FieldElement::new(8,13)?;
        let c = FieldElement::new(3,13)?;

        assert_eq!((a*b)?, c);

        assert_eq!(a.pow(4), c);

        assert_ne!(a.pow(5), c);

        Ok(())
    }

    #[test]
    fn field_element_div() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(2,13)?;
        let b = FieldElement::new(8,13)?;
        let c = FieldElement::new(3,13)?;
        let d = FieldElement::new(4,13)?;

        assert_eq!((b / a)?, d);

        assert_eq!(((a.pow(3) * c)? / b)?, c);

        Ok(())
    }
}