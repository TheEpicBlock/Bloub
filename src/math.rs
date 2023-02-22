use std::ops::{Add, Sub, Neg, Mul, AddAssign, SubAssign, Div};

use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}

impl<T> Vec2<T> where T: From<u8> {
    pub fn zero() -> Self {
        Vec2::new(0.into(), 0.into())
    }
}

impl<T> Add for Vec2<T> where T: Add {
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> AddAssign for Vec2<T> where T: AddAssign {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Vec2<T> where T: Sub {
    type Output = Vec2<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> SubAssign for Vec2<T> where T: SubAssign {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Neg for Vec2<T> where T: Neg {
    type Output = Vec2<T::Output>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl<T> Mul<T> for Vec2<T> where T: Mul + Copy {
    type Output = Vec2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2::new(self.x*rhs, self.y*rhs)
    }
}

impl<T> Div<T> for Vec2<T> where T: Div + Copy {
    type Output = Vec2<T::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Vec2::new(self.x/rhs, self.y/rhs)
    }
}

impl<T, P> From<PhysicalPosition<P>> for Vec2<T> where P: Into<T>{
    fn from(value: PhysicalPosition<P>) -> Self {
        Vec2::new(value.x.into(), value.y.into())
    }
}

impl<T, P> From<PhysicalSize<P>> for Vec2<T> where P: Into<T>{
    fn from(value: PhysicalSize<P>) -> Self {
        Vec2::new(value.width.into(), value.height.into())
    }
}

impl Into<PhysicalPosition<i32>> for Vec2<f64> {
    fn into(self) -> PhysicalPosition<i32> {
        PhysicalPosition::new(self.x as i32, self.y as i32)
    }
}

impl From<Vec2<i64>> for Vec2<f64> {
    fn from(value: Vec2<i64>) -> Self {
        Vec2::new(value.x as f64, value.y as f64)
    }
}

// impl<A,B> From<Vec2<B>> for Vec2<A> where B: From<A> {
//     fn from(value: Vec2<B>) -> Self {
//         todo!()
//     }
// }

// impl<T, P> Into<PhysicalPosition<P>> for Vec2<T> where P: From<T>{
//     fn into(self) -> PhysicalPosition<P> {
//         PhysicalPosition::new(self.x.into(), self.y.into())
//     }
// }

impl Vec2<f64> {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Vec2<f32> {
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Vec2<i64> {
    pub fn len(&self) -> f64 {
        Vec2::<f64>::from(self.clone()).len()
    }
}