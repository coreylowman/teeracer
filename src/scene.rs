use crate::{
    data::{CanHit, Hit, Material, Ray, Three},
    shapes::Object,
};
use num_traits::Float;
use rand::Rng;

pub trait SceneTracer<F> {
    fn trace<R>(ray: Ray<F>, scene: &Scene<F>, depth: usize, rng: &mut R) -> Option<Three<F>>
    where
        R: Rng;
}

pub struct Scene<F> {
    objects: Vec<Object<F>>,
    object_material_idx: Vec<MaterialIdx>,
    materials: Vec<Material<F>>,
}

impl<F> Scene<F> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            object_material_idx: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_material<M: Into<Material<F>>>(&mut self, material: M) -> MaterialIdx {
        let idx = MaterialIdx(self.materials.len());
        self.materials.push(material.into());
        idx
    }

    pub fn add_object<O: Into<Object<F>>>(&mut self, obj: O, mat_idx: MaterialIdx) {
        self.objects.push(obj.into());
        self.object_material_idx.push(mat_idx);
    }

    pub fn material_for(&self, obj_idx: usize) -> &Material<F> {
        let mat_idx = self.object_material_idx[obj_idx];
        &self.materials[mat_idx.0]
    }
}

impl<F> CanHit<Scene<F>, F> for Ray<F>
where
    F: Float,
{
    fn shoot_at(&self, scene: &Scene<F>, t_min: F, mut t_max: F) -> Option<Hit<F>> {
        let mut opt_hit = None;
        for (i, obj) in scene.objects.iter().enumerate() {
            if let Some(mut hit) = self.shoot_at(obj, t_min, t_max) {
                if hit.distance < t_max {
                    hit.object_index = i;
                    opt_hit = Some(hit);
                    t_max = hit.distance;
                }
            }
        }
        opt_hit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialIdx(usize);
