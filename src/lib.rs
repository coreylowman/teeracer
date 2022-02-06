pub mod data;
pub mod hittables;
mod rendering;
pub mod scene;
pub mod tracer;

pub use data::{
    Camera, CanHit, Dielectric, DiffuseLight, Hit, IndexOfRefraction, Lambertian, Material, Metal,
    Ray, Three,
};
pub use hittables::{Plane, Prism, Rectangle, Sphere, Triangle};
pub use rendering::render;
pub use scene::{Scene, SceneTracer};
pub use tracer::PathTracer;
