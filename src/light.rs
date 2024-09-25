use nalgebra_glm::Vec3;
use crate::color::Color;
use std::f32::consts::PI;

pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Light {
            position,
            color,
            intensity,
        }
    }

    // Función para actualizar el ciclo día/noche
    pub fn update_day_night_cycle(&mut self, elapsed_time: f32, day_duration: f32) {
        let time_factor = (elapsed_time % day_duration) / day_duration;
        let light_angle = time_factor * 2.0 * PI;

        // Actualizamos la posición de la luz para simular el movimiento del sol
        self.position.x = light_angle.cos() * 10.0;
        self.position.y = light_angle.sin() * 10.0;

        // Cambiamos el color de la luz según la hora del día
        if time_factor < 0.5 {
            // Simulación de amanecer
            self.color = Color::new(255, 255, 224);  // Luz más cálida (amanecer)
        } else {
            // Simulación de atardecer
            self.color = Color::new(255, 140, 0);  // Luz más anaranjada (atardecer)
        }
    }
}
