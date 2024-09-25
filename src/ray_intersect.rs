use nalgebra_glm::Vec3;
use crate::material::Material;

pub struct Intersect {
    pub is_intersecting: bool,
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersect {
    pub fn empty() -> Self {
        Intersect {
            is_intersecting: false,
            distance: f32::INFINITY,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Material::default(),
        }
    }

    pub fn hit(distance: f32, point: Vec3, normal: Vec3, material: &Material) -> Self {
        Intersect {
            is_intersecting: true,
            distance,
            point,
            normal,
            material: material.clone(),
        }
    }

    pub fn no_hit() -> Self {
        Intersect::empty()
    }
}

pub trait RayIntersect {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect;
}