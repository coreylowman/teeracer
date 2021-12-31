use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Three<T> {
    data: [T; 3],
}

impl<T> Three<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { data: [x, y, z] }
    }
}

impl<T> From<T> for Three<T>
where
    T: Copy,
{
    fn from(t: T) -> Self {
        Self { data: [t, t, t] }
    }
}

impl<T> From<(T, T, T)> for Three<T> {
    fn from(ts: (T, T, T)) -> Self {
        Self {
            data: [ts.0, ts.1, ts.2],
        }
    }
}

impl<T> From<[T; 3]> for Three<T> {
    fn from(ts: [T; 3]) -> Self {
        Self { data: ts }
    }
}

impl<T> Default for Three<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            data: [Default::default(), Default::default(), Default::default()],
        }
    }
}

impl<T> Index<usize> for Three<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Three<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Mul<T> for Three<T>
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

impl<T> MulAssign<T> for Three<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self[i] *= rhs;
        }
    }
}

impl<T> Mul<Three<T>> for Three<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Three<T>) -> Self::Output {
        Self {
            data: [self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]],
        }
    }
}

impl<T> MulAssign for Three<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] *= rhs[i];
        }
    }
}

impl<T> Div<T> for Three<T>
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

impl<T> Neg for Three<T>
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

impl<T> Add for Three<T>
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

impl<T> AddAssign for Three<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self[i] += rhs[i];
        }
    }
}

impl<T> DivAssign<T> for Three<T>
where
    T: DivAssign + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        for i in 0..3 {
            self[i] /= rhs;
        }
    }
}

impl<T> Sub for Three<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl<T> Three<T>
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

impl Length<f32> for Three<f32> {
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

impl Length<f64> for Three<f64> {
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

impl<T> Three<T>
where
    Three<T>: Length<T>,
    T: Div<Output = T> + Copy,
{
    pub fn normalized(&self) -> Self {
        let l = self.length();
        Self {
            data: [self[0] / l, self[1] / l, self[2] / l],
        }
    }
}

impl<T> Three<T>
where
    T: Copy,
{
    pub(crate) fn fill(&mut self, value: T) {
        for i in 0..3 {
            self[i] = value;
        }
    }
}
