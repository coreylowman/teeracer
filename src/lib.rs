pub mod data;
mod data_impls;
pub mod pdf;
mod rendering;
pub mod scene;
pub mod shapes;
pub mod tracer;

pub use data::{
    Camera, CanHit, Dielectric, Diffuse, FieldOfView, Hit, ImageShape, Light, Material, Mirror,
    Ray, Three,
};
pub use rendering::render;
pub use scene::{Scene, SceneTracer};
pub use shapes::{Plane, Prism, Sphere, Triangle};
pub use tracer::PathTracer;
