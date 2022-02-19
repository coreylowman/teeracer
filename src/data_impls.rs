use super::data::*;
use num_traits::{cast, Float};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

impl<T> Debug for Three<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Three")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl<T> Three<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<T> for Three<T>
where
    T: Copy,
{
    fn from(t: T) -> Self {
        Self::new(t, t, t)
    }
}

impl<T> From<(T, T, T)> for Three<T> {
    fn from(ts: (T, T, T)) -> Self {
        Self::new(ts.0, ts.1, ts.2)
    }
}

impl<T> From<[T; 3]> for Three<T>
where
    T: Copy,
{
    fn from(ts: [T; 3]) -> Self {
        Self::new(ts[0], ts[1], ts[2])
    }
}

impl<T> Default for Three<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default(), Default::default())
    }
}

impl<T> Mul<T> for Three<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Mul<T> for &Three<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Three<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Three::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> MulAssign<T> for Three<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<T> Mul<Three<T>> for Three<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: Three<T>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> Mul<&Three<T>> for Three<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;
    fn mul(self, rhs: &Three<T>) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> MulAssign for Three<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> MulAssign<&Self> for Three<T>
where
    T: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: &Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> Div<T> for Three<T>
where
    T: Div<Output = T> + Copy + Float,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let inv_rhs = rhs.recip();
        Self::new(self.x * inv_rhs, self.y * inv_rhs, self.z * inv_rhs)
    }
}

impl<T> Div<T> for &Three<T>
where
    T: Div<Output = T> + Copy + Float,
{
    type Output = Three<T>;
    fn div(self, rhs: T) -> Self::Output {
        let inv_rhs = rhs.recip();
        Three::new(self.x * inv_rhs, self.y * inv_rhs, self.z * inv_rhs)
    }
}

impl<T> DivAssign<T> for Three<T>
where
    T: MulAssign + Copy + Float,
{
    fn div_assign(&mut self, rhs: T) {
        let inv_rhs = rhs.recip();
        self.x *= inv_rhs;
        self.y *= inv_rhs;
        self.z *= inv_rhs;
    }
}

impl<T> Neg for Three<T>
where
    T: Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Add for Three<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Add<&Self> for Three<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Add<Self> for &Three<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Three<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Three::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> AddAssign for Three<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> AddAssign<&Self> for Three<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Sub for Three<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Sub<&Self> for Three<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Sub<Self> for &Three<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Three<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Three::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> SubAssign for Three<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> SubAssign<&Self> for Three<T>
where
    T: SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T> Three<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn length_squared(&self) -> T {
        self.dot(&self)
    }
}

impl<T> Three<T>
where
    T: Mul<Output = T> + Sub<Output = T> + Copy,
{
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl<T> Three<T>
where
    T: Float,
{
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
}

impl<T> Three<T>
where
    T: Float,
    Three<T>: Div<T>,
{
    pub fn normalized(&self) -> Self {
        self / self.length()
    }
}

impl<T> Three<T>
where
    T: Copy,
{
    pub fn fill(&mut self, value: T) {
        self.x = value;
        self.y = value;
        self.z = value;
    }
}

impl<F> Three<F>
where
    F: Float,
{
    pub fn zeros() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    pub fn ones() -> Self {
        Self {
            x: F::one(),
            y: F::one(),
            z: F::one(),
        }
    }

    pub fn rotate(&self, axis: &Self, angle: F) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        self * cos_theta
            + (self.cross(axis) * sin_theta + axis * (self.dot(axis) * (F::one() - cos_theta)))
    }
}

impl<F> Diffuse<F> {
    pub fn rgb(r: F, g: F, b: F) -> Self {
        Self {
            rgb: Three::new(r, g, b),
        }
    }
}

impl<F> Mirror<F>
where
    F: Float,
{
    pub fn perfect() -> Self {
        Self { rgb: Three::ones() }
    }

    pub fn tinted(r: F, g: F, b: F) -> Self {
        Self {
            rgb: Three::new(r, g, b),
        }
    }
}

impl<F> Dielectric<F>
where
    F: Float,
{
    pub fn transparent(ior: F) -> Self {
        Self {
            rgb: Three::ones(),
            ior,
        }
    }

    pub fn tint(self, r: F, g: F, b: F) -> Self {
        Self {
            rgb: Three::new(r, g, b),
            ior: self.ior,
        }
    }
}

impl<F> Material<F> {
    pub fn is_emissive(&self) -> bool {
        match self {
            Material::Light(_) => true,
            _ => false,
        }
    }
}

impl<F> Into<Material<F>> for Diffuse<F> {
    fn into(self) -> Material<F> {
        Material::Diffuse(self)
    }
}

impl<F> Into<Material<F>> for Mirror<F> {
    fn into(self) -> Material<F> {
        Material::Mirror(self)
    }
}

impl<F> Into<Material<F>> for Dielectric<F> {
    fn into(self) -> Material<F> {
        Material::Dielectric(self)
    }
}

impl<F> Into<Material<F>> for Light<F> {
    fn into(self) -> Material<F> {
        Material::Light(self)
    }
}

impl<F> LinearTransform<F>
where
    F: Float,
{
    pub fn apply(&self, x: F) -> F {
        self.scale * x + self.offset
    }
}

impl<F> FieldOfView<F>
where
    F: Float,
{
    pub fn radians(self) -> F {
        match self {
            Self::Degrees(v) => v.to_radians(),
            Self::Radians(v) => v,
        }
    }
}

impl<F> Camera<F>
where
    F: Float,
{
    pub fn new(fov: FieldOfView<F>, image_shape: ImageShape) -> Self {
        let two: F = F::from(2.0f64).unwrap();
        let tan_half_fov = (fov.radians() / two).tan();
        let w: F = cast(image_shape.width).unwrap();
        let h: F = cast(image_shape.height).unwrap();
        let aspect_ratio = w / h;
        Self {
            position: Three::new(F::zero(), F::zero(), F::zero()),
            x_transform: LinearTransform {
                // (2.0 * x / width - 1.0) * aspect_ratio & tan_half_fov
                scale: two * aspect_ratio * tan_half_fov / w,
                offset: -aspect_ratio * tan_half_fov,
            },
            y_transform: LinearTransform {
                // (1.0 - 2.0 * y / height) * tanh_half_fov
                scale: -two * tan_half_fov / h,
                offset: tan_half_fov,
            },
            width: image_shape.width,
            height: image_shape.height,
        }
    }

    pub fn at(&self, x: F, y: F, z: F) -> Self {
        let mut p = self.clone();
        p.position = Three::new(x, y, z);
        p
    }

    pub(crate) fn empty_image(&self) -> Vec<Three<F>> {
        vec![Three::new(F::zero(), F::zero(), F::zero()); self.width * self.height]
    }

    pub(crate) fn ray_through(&self, x_screen: F, y_screen: F) -> Ray<F> {
        let x_world = self.x_transform.apply(x_screen);
        let y_world = self.y_transform.apply(y_screen);
        let direction = Three::new(x_world, y_world, -F::one()).normalized();
        Ray {
            origin: self.position,
            direction,
        }
    }
}
