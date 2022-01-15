use num_traits::Float;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy)]
pub struct Three<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

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

impl Three<f64> {
    pub fn min(&self, other: &Self) -> Self {
        Three::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn max(&self, other: &Self) -> Self {
        Three::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}

impl Three<f64> {
    pub fn rotate(&self, axis: &Self, angle: f64) -> Self {
        let theta = angle.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        self * cos_theta
            + (self.cross(axis) * sin_theta + axis * (self.dot(axis) * (1.0 - cos_theta)))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub rgb: Three<f64>,
}

impl Lambertian {
    pub fn new(rgb: Three<f64>) -> Self {
        Self { rgb }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub rgb: Three<f64>,
    pub fuzz: Option<f64>,
}

impl Metal {
    pub fn new(rgb: Three<f64>) -> Self {
        Self { rgb, fuzz: None }
    }

    pub fn fuzzy(rgb: Three<f64>, fuzz: f64) -> Self {
        Self {
            rgb,
            fuzz: Some(fuzz),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub ior: IndexOfRefraction,
}

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub rgb: Three<f64>,
    pub power: f64,
}

impl DiffuseLight {
    pub fn new(rgb: Three<f64>, power: f64) -> Self {
        Self { rgb, power }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl Into<Material> for Lambertian {
    fn into(self) -> Material {
        Material::Lambertian(self)
    }
}

impl Into<Material> for Metal {
    fn into(self) -> Material {
        Material::Metal(self)
    }
}

impl Into<Material> for Dielectric {
    fn into(self) -> Material {
        Material::Dielectric(self)
    }
}

impl Into<Material> for DiffuseLight {
    fn into(self) -> Material {
        Material::DiffuseLight(self)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum IndexOfRefraction {
    Vacuum,
    Air,
    Ice,
    Water,
    CrownGlass,
    Diamond,
}

impl Into<Material> for IndexOfRefraction {
    fn into(self) -> Material {
        Material::Dielectric(Dielectric { ior: self })
    }
}

impl IndexOfRefraction {
    pub const fn value(&self) -> f64 {
        match self {
            Self::Vacuum => 1.0,
            Self::Air => 1.00029,
            Self::Ice => 1.31,
            Self::Water => 1.33,
            Self::CrownGlass => 1.52,
            Self::Diamond => 2.417,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Three<f64>,
    pub direction: Three<f64>,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            direction: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hit {
    pub position: Three<f64>,
    pub distance: f64,
    pub normal: Three<f64>,
}

pub trait CanHit {
    fn hit_by(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
