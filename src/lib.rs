pub mod camera;
pub mod data;
pub mod hittables;
pub mod scene;
pub mod tracer;

pub use camera::Camera;
pub use data::{
    CanHit, Dielectric, DiffuseLight, Hit, IndexOfRefraction, Lambertian, Material, Metal, Ray,
    Three,
};
pub use hittables::{Plane, Prism, Rectangle, Sphere, Triangle};
pub use scene::Scene;
pub use tracer::{PathTracer, Tracer};
