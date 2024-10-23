use std::{ops, process::Output};
use num_traits;

#[derive(Debug)]
struct InvalidFieldElement;

impl std::error::Error for InvalidFieldElement {}

impl std::fmt::Display for InvalidFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Num is not in field range 0 to (Order - 1)")
    }
}

#[derive(Debug)]
struct DifferentOrderExpression;

impl std::error::Error for DifferentOrderExpression {}

impl std::fmt::Display for DifferentOrderExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannnot add two numbers in different Fields")
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<FieldElement, InvalidFieldElement> {
        if num >= prime || num < 0 || prime < 0 {
            return Err(InvalidFieldElement);
        }

        Ok(FieldElement {
            num: num,
            prime: prime,
        })
    }
}

trait Modulus {
    fn modulus(&self, other: Self) -> Self;
}

impl<T: num_traits::PrimInt> Modulus for T {
    fn modulus(&self, other: Self) -> Self {
        ((*self % other) + other) % other
    }
}

impl ops::Add for FieldElement {
    type Output = Result<Self, DifferentOrderExpression>;

    fn add(self, rhs: Self) ->  Self::Output{
        if self.prime != rhs.prime { return Err(DifferentOrderExpression) }
        
        Ok(Self { 
            num: (self.num + rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }
}

impl ops::Sub for FieldElement {
    type Output = Result<Self, DifferentOrderExpression>;

    fn sub(self, rhs: Self) ->  Self::Output{
        if self.prime != rhs.prime { return Err(DifferentOrderExpression) }
        
        Ok(Self { 
            num: (self.num - rhs.num).modulus(self.prime),
            prime: self.prime
        })
    }
}

fn main() {
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
    fn field_element_add() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(7,13)?;
        let b = FieldElement::new(12,13)?;
        let c = FieldElement::new(6,13)?;

        assert_eq!((a+b)?, c);

        Ok(())
    }

    #[test]
    fn field_element_sub() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(9,57)?;
        let b = FieldElement::new(29,57)?;
        let c = FieldElement::new(37,57)?;

        assert_eq!((a-b)?, c);

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
}