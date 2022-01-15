use crate::{
    hittables::Object,
    material::Material,
    ray::{CanHit, Hit, Ray},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialIdx(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectIdx(usize);

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

    pub fn add_object<O: Into<Object>>(&mut self, obj: O, mat_idx: MaterialIdx) -> ObjectIdx {
        let obj_idx = ObjectIdx(self.objects.len());
        self.objects.push(obj.into());
        self.object_material_idx.push(mat_idx);
        obj_idx
    }

    fn material_for(&self, obj_idx: usize) -> &Material {
        let mat_idx = self.object_material_idx[obj_idx];
        &self.materials[mat_idx.0]
    }

    pub fn cast(&self, ray: &Ray) -> Option<(Hit, &Material)> {
        let mut opt_hit = None;
        let t_min = 1e-3;
        let mut t_max = f64::INFINITY;
        for (i, obj) in self.objects.iter().enumerate() {
            if let Some(hit) = obj.hit_by(&ray, t_min, t_max) {
                if hit.distance < t_max {
                    opt_hit = Some((hit, i));
                    t_max = hit.distance;
                }
            }
        }
        opt_hit.map(|(h, i)| (h, self.material_for(i)))
    }
}
