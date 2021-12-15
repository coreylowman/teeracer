use std::{
    iter::Sum,
    ops::{Add, Div, Index, Mul, Sub},
};

#[derive(Clone, Copy)]
pub struct Vec3<T = f64> {
    data: [T; 3],
}

impl<T> From<T> for Vec3<T>
where
    T: Copy,
{
    fn from(value: T) -> Self {
        Self {
            data: [value, value, value],
        }
    }
}
impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(t: (T, T, T)) -> Self {
        Self {
            data: [t.0, t.1, t.2],
        }
    }
}

impl<T> Default for Vec3<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: [Default::default(), Default::default(), Default::default()],
        }
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            data: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            data: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sum<T> + Copy,
{
    pub fn dot(&self, other: &Self) -> T {
        (0..3).map(|i| self[i] * other[i]).sum()
    }
}

pub trait Length<T> {
    fn length(&self) -> T;
}

impl Length<f32> for Vec3<f32> {
    fn length(&self) -> f32 {
        self.dot(&self).sqrt()
    }
}

impl Length<f64> for Vec3<f64> {
    fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }
}

impl<T> Vec3<T>
where
    Vec3<T>: Length<T>,
    T: Div<Output = T> + Copy,
{
    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {
            data: [self[0] / l, self[1] / l, self[2] / l],
        }
    }
}
