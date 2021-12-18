use rand::{prelude::Distribution, thread_rng};
use rand_distr::{Standard, StandardNormal};
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub},
};

#[derive(Clone, Copy)]
pub struct Vec3<T = f64> {
    data: [T; 3],
}

impl Vec3<f64> {
    pub(crate) fn near_zero(&self) -> bool {
        const Z: f64 = 1e-6;
        self[0].abs() < Z && self[1] < Z && self[2] < Z
    }
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

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            data: [self[0] * rhs, self[1] * rhs, self[2] * rhs],
        }
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            data: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self {
            data: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            data: [-self[0], -self[1], -self[2]],
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl<T> AddAssign for Vec3<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] += rhs[i];
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
    Standard: Distribution<T>,
    StandardNormal: Distribution<T>,
    Vec3<T>: Length<T>,
    T: Div<Output = T> + Mul<Output = T> + Copy,
{
    pub(crate) fn random_unit() -> Self {
        let mut rng = thread_rng();
        Self {
            data: [
                StandardNormal.sample(&mut rng),
                StandardNormal.sample(&mut rng),
                StandardNormal.sample(&mut rng),
            ],
        }
        .normalized()
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sum<T> + Copy,
{
    pub(crate) fn dot(&self, other: &Self) -> T {
        (0..3).map(|i| self[i] * other[i]).sum()
    }
}

impl Vec3<f64> {
    pub(crate) fn reflect(&self, normal: &Self) -> Self {
        *self - (*normal * self.dot(normal)) * 2.0
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
