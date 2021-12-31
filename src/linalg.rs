use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T = f64> {
    data: [T; 3],
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { data: [x, y, z] }
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

impl<T> MulAssign<T> for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self[i] *= rhs;
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

impl<T> MulAssign for Vec3<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] *= rhs[i];
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

impl<T> DivAssign<T> for Vec3<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self[i] /= rhs;
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
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    pub(crate) fn dot(&self, rhs: &Self) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

pub trait Length<T> {
    fn length_squared(&self) -> T;
    fn length(&self) -> T;
    fn is_unit(&self) -> bool;
}

impl Length<f32> for Vec3<f32> {
    fn length_squared(&self) -> f32 {
        self.dot(&self)
    }
    fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    fn is_unit(&self) -> bool {
        (self.length() - 1.0).abs() <= 1e-6
    }
}

impl Length<f64> for Vec3<f64> {
    fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn is_unit(&self) -> bool {
        (self.length() - 1.0).abs() <= 1e-6
    }
}

impl<T> Vec3<T>
where
    Vec3<T>: Length<T>,
    T: Div<Output = T> + DivAssign + Copy,
{
    pub fn normalize(&mut self) {
        let l = self.length();
        *self /= l;
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

impl<T> Vec3<T>
where
    T: Copy,
{
    pub(crate) fn fill(&mut self, value: T) {
        for i in 0..3 {
            self[i] = value;
        }
    }
}
