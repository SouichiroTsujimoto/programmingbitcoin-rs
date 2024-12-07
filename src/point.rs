use std::{ops};
use crate::field_element::{ExpressionError, FieldElement, FieldElementOperation};

#[derive(Debug)]
pub struct Point<T>
where
    T: FieldElementOperation,
{
    x: Option<T>,
    y: Option<T>,
    a: T,
    b: T,
}

impl Point<FieldElement> {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Result<Self, ExpressionError> {
        if x.is_none() || y.is_none() {
            return Ok(Self { x, y, a, b });
        }

        let x = x.unwrap();
        let y = y.unwrap();
        
        let ax = match a * x {
            Ok(result) => result,
            Err(_) => return Err(ExpressionError::InvalidPoint),
        };
        let x_pow_3_add_ax = match x.pow(3) + ax {
            Ok(result) => result,
            Err(_) => return Err(ExpressionError::InvalidPoint),
        };
        let x_pow_3_add_ax_add_b = match x_pow_3_add_ax + b {
            Ok(result) => result,
            Err(_) => return Err(ExpressionError::InvalidPoint),
        };
        
        if y.pow(2) != x_pow_3_add_ax_add_b {
            return Err(ExpressionError::InvalidPoint);
        }
        Ok(Self { x: Some(x), y: Some(y), a, b })
    }
}

pub trait PointOperation {
    type Output;
    fn add_op(&self, rhs: &Self) -> Self::Output;
    fn mul_op(&self, rhs: u32) -> Self::Output;
}
impl PointOperation for Point<FieldElement> {
    type Output = Result<Self, ExpressionError>;

    fn add_op(&self, rhs: &Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            return Err(ExpressionError::DifferentCurves);
        }

        if self.x.is_none() || self.y.is_none() {
            return Ok(rhs.clone());
        }
        if rhs.x.is_none() || rhs.y.is_none() {
            return Ok(self.clone());
        }

        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();

        let x1_sub_x2 = (x1 - x2)?;
        let y1_sub_y2 = (y1 - y2)?;
        let x1_add_x2 = (x1 + x2)?;

        if x1 == x2 && y1 != y2 {
            // y軸対称

            return Ok(Point { x: None, y: None, a: self.a, b: self.b });
        } else if x1 == x2 && y1 == y2 {
            // 同じ点同士の加算 -> 接線
            
            let three = FieldElement::new(3, x1.prime).unwrap();
            let two = FieldElement::new(2, x1.prime).unwrap();
            let zero = FieldElement::new(0, x1.prime).unwrap();

            // 接線が垂直
            if y1 == zero {
                return Ok(Point::new(None, None, self.a, self.b).unwrap());
            }

            let s = (((three * x1.pow(2))? + self.a)? / (two * y1)?)?;
            // s^2 - x1 - x2 (分配法則が成り立つので、-(x1+x2)としている)
            let x3 = (s.pow(2) - x1_add_x2)?;
            let y3 = ((s * (x1 - x3)?)? - y1)?;
            
            Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?)
        } else {
            // x値の異なる点同士の加算

            let s = (y1_sub_y2 / x1_sub_x2)?;
            // s^2 - x1 - x2 (分配法則が成り立つので、-(x1+x2)としている)
            let x3 = (s.pow(2) - x1_add_x2)?;
            let y3 = ((s * (x1 - x3)?)? - y1)?;
            
            Ok(Point::new(Some(x3), Some(y3), self.a, self.b)?)
        }
    }

    fn mul_op(&self, rhs: u32) -> Self::Output {
        let mut res = Point::new(None, None, self.a, self.b).unwrap();

        let mut coef = rhs;
        let mut current = self.clone();

        while coef > 0 {
            if coef & 1 == 1 {
                res = (&res + &current)?;
            }
            current = (&current + &current)?;
            coef >>= 1;
        }
        Ok(res)
    }
}

impl Clone for Point<FieldElement> {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y, a: self.a, b: self.b }
    }
}

impl PartialEq for Point<FieldElement> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y
    }
}

impl ops::Add<&Point<FieldElement>> for &Point<FieldElement> {
    type Output = Result<Point<FieldElement>, ExpressionError>;

    fn add(self, rhs: &Point<FieldElement>) -> Self::Output {
        self.add_op(rhs)
    }
}


impl ops::Mul<u32> for &Point<FieldElement> {
    type Output = Result<Point<FieldElement>, ExpressionError>;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul_op(rhs)
    }
}

impl ops::Mul<&Point<FieldElement>> for u32 {
    type Output = Result<Point<FieldElement>, ExpressionError>;

    fn mul(self, rhs: &Point<FieldElement>) -> Self::Output {
        rhs * self  // 既に実装した Point * u32 を再利用
    }
}