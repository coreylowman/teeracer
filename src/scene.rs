use crate::{
    data::{CanHit, Hit, Material, Ray, Three},
    shapes::Object,
};
use rand::Rng;

pub trait SceneTracer {
    fn trace<R>(ray: Ray, scene: &Scene, depth: usize, rng: &mut R) -> Option<Three<f64>>
    where
        R: Rng;
}

pub struct Scene {
    objects: Vec<Object>,
    object_material_idx: Vec<MaterialIdx>,
    materials: Vec<Material>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            object_material_idx: Vec::new(),
            materials: Vec::new(),
        }
    }

    pub fn add_material<M: Into<Material>>(&mut self, material: M) -> MaterialIdx {
        let idx = MaterialIdx(self.materials.len());
        self.materials.push(material.into());
        idx
    }

    pub fn add_object<O: Into<Object>>(&mut self, obj: O, mat_idx: MaterialIdx) {
        self.objects.push(obj.into());
        self.object_material_idx.push(mat_idx);
    }

    pub fn material_for(&self, obj_idx: usize) -> &Material {
        let mat_idx = self.object_material_idx[obj_idx];
        &self.materials[mat_idx.0]
    }
}

impl CanHit<Scene> for Ray {
    fn shoot_at(&self, scene: &Scene, t_min: f64, mut t_max: f64) -> Option<Hit> {
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
