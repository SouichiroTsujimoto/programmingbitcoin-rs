#[derive(Debug)]
struct InvalidFieldElement;

impl std::error::Error for InvalidFieldElement {}

impl std::fmt::Display for InvalidFieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Num is not in field range 0 to (Prime - 1)")
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Result<FieldElement, InvalidFieldElement> {
        if num >= prime {
            return Err(InvalidFieldElement);
        }

        Ok(FieldElement {
            num: num,
            prime: prime,
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

        let b = FieldElement::new(30, 29);

        match b {
            Ok(_) => panic!("The expected is Err."),
            Err(ife) => ife,
        };
        assert!(true);

        let c = FieldElement::new(9, 29)?;
        let d = FieldElement::new(10, 31)?;

        assert!(a != c);
        assert!(a != d);
        

        Ok(())
    }

    fn field_element_modulo() {
        
    }
}