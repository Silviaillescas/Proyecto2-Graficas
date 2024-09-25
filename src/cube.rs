use nalgebra_glm::{Vec3, dot};
use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(1.0 / direction.x, 1.0 / direction.y, 1.0 / direction.z);
        
        let mut tmin = (self.min.x - origin.x) * inv_dir.x;
        let mut tmax = (self.max.x - origin.x) * inv_dir.x;
        
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }

        let mut tymin = (self.min.y - origin.y) * inv_dir.y;
        let mut tymax = (self.max.y - origin.y) * inv_dir.y;

        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::no_hit();
        }

        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (self.min.z - origin.z) * inv_dir.z;
        let mut tzmax = (self.max.z - origin.z) * inv_dir.z;

        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::no_hit();
        }

        let distance = if tzmin > tmin { tzmin } else { tmin };

        let hit_point = origin + direction * distance;
        let normal = (hit_point - (self.max + self.min) * 0.5).normalize();

        Intersect::hit(distance, hit_point, normal, &self.material)
    }
}
