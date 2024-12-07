use std::ops;
use crate::field_element::{FieldElement, FieldElementOperation};
use crate::field_element::{ExpressionError, Modulus};
use crate::point::{Point, PointOperation};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S256Field {
    pub num: i32,
    pub prime: u32,
}

impl S256Field { 
    pub const fn new(num: i32) -> Self {
        S256Field {
            num: num,
            prime: 2u32.pow(256) - 2u32.pow(32) - 977,
        }
    }
}

impl FieldElementOperation for S256Field {
    type GeneralOpOutput = Self;

    fn add_op(self, rhs: Self) -> Self::GeneralOpOutput {
        Self { 
            num: (self.num + rhs.num).modulus(self.prime),
            prime: self.prime
        }
    }

    fn sub_op(self, rhs: Self) -> Self::GeneralOpOutput {
        Self { 
            num: (self.num - rhs.num).modulus(self.prime),
            prime: self.prime
        }
    }

    fn mul_op(self, rhs: Self) -> Self::GeneralOpOutput {
        Self { 
            num: (self.num * rhs.num).modulus(self.prime),
            prime: self.prime
        }
    }

    fn div_op(self, rhs: Self) -> Self::GeneralOpOutput {
        self.mul_op(rhs.pow(-1))
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

impl ops::Add for S256Field {
    type Output = Self;

    fn add(self, rhs: Self) ->  Self::Output{
        self.add_op(rhs)
    }
}

impl ops::Sub for S256Field {
    type Output = Self;

    fn sub(self, rhs: Self) ->  Self::Output{
        self.sub_op(rhs)
    }
}

impl ops::Mul for S256Field {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output{
        self.mul_op(rhs)
    }
}

impl ops::Div for S256Field {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.div_op(rhs)
    }
}


impl std::fmt::Display for S256Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:064}", self.num)
    }
}



#[derive(Debug)]
pub struct S256Point<T>
where
    T: FieldElementOperation,
{
    x: Option<T>,
    y: Option<T>,
    a: T,
    b: T,
}

impl Clone for S256Point<S256Field> {
    fn clone(&self) -> Self {
        S256Point { x: self.x, y: self.y, a: self.a, b: self.b }
    }
}

impl PartialEq for S256Point<S256Field> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl S256Point<S256Field> {
    const a: S256Field = S256Field::new(0);
    const b: S256Field = S256Field::new(7);
    const N1: u128 = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE;
    const N2: u128 = 0xBAAEDCE6AF48A03BBFD25E8CD0364141;
    
    
    
    
    
    // TODO: BigInt U256






    

    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> Result<Self, ExpressionError> {
        if x.is_none() || y.is_none() {
            return Ok(Self { x, y, a: Self::a, b: Self::b });
        }

        let x = x.unwrap();
        let y = y.unwrap();
        
        let ax = Self::a * x;
        let x_pow_3_add_ax = x.pow(3) + ax;
        let x_pow_3_add_ax_add_b = x_pow_3_add_ax + Self::b;
        
        if y.pow(2) != x_pow_3_add_ax_add_b {
            return Err(ExpressionError::InvalidPoint);
        }
        Ok(Self { x: Some(x), y: Some(y), a: Self::a, b: Self::b })
    }
}

impl PointOperation for S256Point<S256Field> {
    type Output = Self;

    fn add_op(&self, rhs: &Self) -> Self::Output {
        if self.x.is_none() || self.y.is_none() {
            rhs.clone();
        }
        if rhs.x.is_none() || rhs.y.is_none() {
            self.clone();
        }

        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();

        let x1_sub_x2 = x1 - x2;
        let y1_sub_y2 = y1 - y2;
        let x1_add_x2 = x1 + x2;

        if x1 == x2 && y1 != y2 {
            // y軸対称
            S256Point::new(None, None).unwrap()
        } else if x1 == x2 && y1 == y2 {
            // 同じ点同士の加算 -> 接線
            let three = S256Field::new(3);
            let two = S256Field::new(2);
            let zero = S256Field::new(0);

            // 接線が垂直
            if y1 == zero {
                S256Point::new(None, None).unwrap();
            }

            let s = (three * x1.pow(2) + self.a) / (two * y1);
            // s^2 - x1 - x2 (分配法則が成り立つので、-(x1+x2)としている)
            let x3 = s.pow(2) - x1_add_x2;
            let y3 = (s * (x1 - x3)) - y1;
            
            S256Point::new(Some(x3), Some(y3)).unwrap()
        } else {
            // x値の異なる点同士の加算

            let s = y1_sub_y2 / x1_sub_x2;
            // s^2 - x1 - x2 (分配法則が成り立つので、-(x1+x2)としている)
            let x3 = s.pow(2) - x1_add_x2;
            let y3 = (s * (x1 - x3)) - y1;
            
            S256Point::new(Some(x3), Some(y3)).unwrap()
        }
    }

    fn mul_op(&self, rhs: u32) -> Self::Output {
        let mut res = S256Point::new(None, None).unwrap();

        let mut coef = rhs / N; // TODO: U256
        let mut current = self.clone();

        while coef > 0 {
            if coef & 1 == 1 {
                res = res.add_op(&current);
            }
            current = current.add_op(&current);
            coef >>= 1;
        }
        res
    }
}

impl ops::Add<&S256Point<S256Field>> for &S256Point<S256Field> {
    type Output = S256Point<S256Field>;

    fn add(self, rhs: &S256Point<S256Field>) -> Self::Output {
        self.add_op(rhs)
    }
}


impl ops::Mul<u32> for &S256Point<S256Field> {
    type Output = S256Point<S256Field>;

    fn mul(self, rhs: u32) -> Self::Output {
        self.mul_op(rhs)
    }
}

impl ops::Mul<&S256Point<S256Field>> for u32 {
    type Output = S256Point<S256Field>;

    fn mul(self, rhs: &S256Point<S256Field>) -> Self::Output {
        rhs * self  // 既に実装した Point * u32 を再利用
    }
}