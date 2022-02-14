use crate::data::Three;
use num_traits::{Float, FloatConst};
use rand::Rng;
use rand_distr::{Distribution, Standard};

pub trait PDF<F> {
    fn sample<R: Rng>(&self, rng: &mut R) -> Three<F>;
    fn value(&self, v: &Three<F>) -> F;
}

pub struct CosinePDF<F> {
    normal: Three<F>,
    u: Three<F>,
    v: Three<F>,
}

impl<F> CosinePDF<F>
where
    F: Float,
{
    pub fn oriented_towards(normal: Three<F>) -> Self {
        let a = if normal.x.abs() > F::from(0.9f64).unwrap() {
            Three::new(F::zero(), F::one(), F::zero())
        } else {
            Three::new(F::one(), F::zero(), F::zero())
        };

        let u = normal.cross(&a).normalized();
        let v = normal.cross(&u).normalized();

        Self { normal, u, v }
    }
}

impl<F> PDF<F> for CosinePDF<F>
where
    F: Float + FloatConst,
    Standard: Distribution<F>,
{
    fn sample<R: Rng>(&self, rng: &mut R) -> Three<F> {
        // sample local random cosine direction
        let r = Standard.sample(rng);
        let z = (F::one() - r).sqrt();
        let phi = F::from(2.0f64).unwrap() * F::PI() * Standard.sample(rng);
        let y = phi.sin() * r.sqrt();
        let x = phi.cos() * r.sqrt();

        // transform to world coordinates using u/v/normal basis
        &self.u * x + &self.v * y + &self.normal * z
    }

    fn value(&self, v: &Three<F>) -> F {
        // abs(cos_theta / pi)
        (v.dot(&self.normal) * F::FRAC_1_PI()).abs()
    }
}
