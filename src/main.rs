mod field_element;
use field_element::{FieldElement, FieldElementOperation};

mod point;
use point::Point;

mod secp256k1;
use secp256k1::S256Field;

fn main() {
    
    let a = FieldElement::new(2,13).unwrap();
    let b = FieldElement::new(4,13).unwrap();
    
    println!("{:#?}", b.pow(2) * a.pow(-4));
    
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use core::panic;

    use crate::field_element::FieldElement;
    use crate::point::Point;

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

    #[test]
    fn ecc_test_valid_points() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(0, 223)?;
        let b = FieldElement::new(7, 223)?;

        let x1 = FieldElement::new(192, 223)?;
        let y1 = FieldElement::new(105, 223)?;
        let x2 = FieldElement::new(17, 223)?;
        let y2 = FieldElement::new(56, 223)?;
        let x3 = FieldElement::new(1, 223)?;
        let y3 = FieldElement::new(193, 223)?;

        Point::new(Some(x1), Some(y1), a, b)?;
        Point::new(Some(x2), Some(y2), a, b)?;
        Point::new(Some(x3), Some(y3), a, b)?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn ecc_test_invalid_points() {
        let a = FieldElement::new(0, 223).unwrap();
        let b = FieldElement::new(7, 223).unwrap();

        let x1 = FieldElement::new(200, 223).unwrap();
        let y1 = FieldElement::new(119, 223).unwrap();
        let x2 = FieldElement::new(42, 223).unwrap();
        let y2 = FieldElement::new(99, 223).unwrap();

        match Point::new(Some(x1), Some(y1), a, b) {
            Ok(_) => println!("why?"),
            Err(_) => panic!("Invalid point"),
        }

        match Point::new(Some(x2), Some(y2), a, b) {
            Ok(_) => println!("why?"),
            Err(_) => panic!("Invalid point"),
        }
    }

    #[test]
    fn ecc_test_add_points() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(0, 223)?;
        let b = FieldElement::new(7, 223)?;

        // 加算テスト1
        let x1 = FieldElement::new(170, 223)?;
        let y1 = FieldElement::new(142, 223)?;
        let x2 = FieldElement::new(60, 223)?;
        let y2 = FieldElement::new(139, 223)?;
        let x3 = FieldElement::new(220, 223)?;
        let y3 = FieldElement::new(181, 223)?;
        
        let p1 = Point::new(Some(x1), Some(y1), a, b)?;
        let p2 = Point::new(Some(x2), Some(y2), a, b)?;
        let p3 = Point::new(Some(x3), Some(y3), a, b)?;

        assert_eq!((&p1 + &p2)?, p3);

        // 加算テスト2
        let x1 = FieldElement::new(47, 223)?;
        let y1 = FieldElement::new(71, 223)?;
        let x2 = FieldElement::new(17, 223)?;
        let y2 = FieldElement::new(56, 223)?;
        let x3 = FieldElement::new(215, 223)?;
        let y3 = FieldElement::new(68, 223)?;

        let p1 = Point::new(Some(x1), Some(y1), a, b)?;
        let p2 = Point::new(Some(x2), Some(y2), a, b)?;
        let p3 = Point::new(Some(x3), Some(y3), a, b)?;

        assert_eq!((&p1 + &p2)?, p3);
        
        Ok(())
    }

    #[test]
    fn ecc_test_mul_points() -> Result<(), Box<dyn std::error::Error>> {
        let a = FieldElement::new(0, 223)?;
        let b = FieldElement::new(7, 223)?;

        let x1 = FieldElement::new(170, 223)?;
        let y1 = FieldElement::new(142, 223)?;

        let p1 = Point::new(Some(x1), Some(y1), a, b)?;

        let p2 = (&p1 * 2)?;

        assert_eq!(p2, (&p1 + &p1)?);
        
        Ok(())
    }
}
