use crate::color::Color;
#[derive(Clone)]  // Asegúrate de que `Material` implemente Clone si aún no lo has hecho
pub struct Material {
    pub color: Color,
    pub shininess: f32,
    pub albedo: [f32; 4],
    pub refractive_index: f32,
    pub name: String,  // Nuevo campo para el nombre del material
}

impl Material {
    pub fn new(color: Color, shininess: f32, albedo: [f32; 4], refractive_index: f32, name: &str) -> Material {
        Material {
            color,
            shininess,
            albedo,
            refractive_index,
            name: name.to_string(),  // Asignar el nombre al material
        }
    }

    pub fn black() -> Material {
        Material::new(Color::black(), 0.0, [0.0, 0.0, 0.0, 0.0], 0.0, "black")
    }
}

// Implementa el trait Default
impl Default for Material {
    fn default() -> Self {
        Material::new(Color::black(), 0.0, [0.0, 0.0, 0.0, 0.0], 0.0, "default")
    }
}
