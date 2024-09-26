mod framebuffer;
mod ray_intersect;
mod sphere;
mod color;
mod camera;
mod light;
mod material;
mod cube;

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec3, normalize};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

use crate::color::Color;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::cube::Cube;

const ORIGIN_BIAS: f32 = 1e-4;
const AMBIENT_LIGHT_DAY: f32 = 0.1;
const AMBIENT_LIGHT_NIGHT: f32 = 0.05;
const DAY_DURATION: f32 = 10.0;  // Duración del ciclo de día y noche en segundos

fn offset_origin(intersect: &Intersect, direction: &Vec3) -> Vec3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(&intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);
    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);
    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
}

fn block_texture(u: f32, v: f32) -> Color {
    let block_size = 10.0;  // Aumentamos el tamaño de los bloques para hacerlos más visibles
    let u_block = ((u * block_size).abs().floor() as i32) % 2;
    let v_block = ((v * block_size).abs().floor() as i32) % 2;

    // Alternamos los colores de acuerdo a las coordenadas u y v
    if (u_block + v_block) % 2 == 0 {
        Color::new(255, 255, 255)  // Color claro
    } else {
        Color::new(0, 0, 0)  // Color oscuro
    }
}


fn realistic_texture(u: f32, v: f32) -> Color {
    let r = (u * 255.0).abs() as u8;
    let g = (v * 255.0).abs() as u8;
    let b = (u * v * 255.0).abs() as u8;
    Color::new(r, g, b)  // Genera una textura más suave y realista
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let light_distance = (light.position - intersect.point).magnitude();
    let shadow_ray_origin = offset_origin(intersect, &light_dir);
    let mut shadow_intensity = 0.0;

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            let distance_ratio = shadow_intersect.distance / light_distance;
            shadow_intensity = 1.0 - distance_ratio.powf(2.0).min(1.0);
            break;
        }
    }

    shadow_intensity
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
    ambient_light: f32,
    depth: u32,
    time: f32,  
) -> Color {
    if depth > 3 {
        return skybox(ray_direction);  // Llamamos a la función skybox aquí
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        return skybox(ray_direction);  // Si no hay intersección, devuelve el skybox
    }

    let light_dir = (light.position - intersect.point).normalize();
    let view_dir = (ray_origin - intersect.point).normalize();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalize();
    
    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);
    
    let diffuse_intensity = intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = light.color * intersect.material.albedo[0] * diffuse_intensity * light_intensity;
    
    let specular_intensity = view_dir.dot(&reflect_dir).max(0.0).powf(intersect.material.shininess);
    let specular = light.color * intersect.material.albedo[1] * specular_intensity * light_intensity;
    
    let mut reflect_color = Color::black();
    let reflectivity = intersect.material.albedo[2] * 0.8;
    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &intersect.normal).normalize();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, ambient_light, depth + 1, time);
    }
    
    let mut refract_color = Color::black();
    let transparency = intersect.material.albedo[3] * 0.9;
    if transparency > 0.0 {
        let refract_dir = refract(&ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, ambient_light, depth + 1, time);
    }

    let (u, v) = uv_mapping(&intersect);

    // Aplicar una textura tipo bloque solo al suelo
    let texture_color = if intersect.material.name == "suelo" {
        block_texture(u, v)  // Textura de bloques para el suelo
    } else if intersect.material.name == "sol" {
        Color::new(255, 223, 0)  // Color amarillo sin textura para el sol
    } else {
        realistic_texture(u, v)  // Textura suave para el muñeco
    };

    let final_color = (texture_color + diffuse + specular) * (1.0 - reflectivity - transparency)
    + (reflect_color * reflectivity)
    + (refract_color * transparency)
    + Color::new(255, 255, 255) * ambient_light;

    final_color
}

fn skybox(ray_direction: &Vec3) -> Color {
    let t = 0.5 * (ray_direction.y + 1.0);
    let top_color = Color::new(135, 206, 250); 
    let bottom_color = Color::new(25, 25, 112); 
    
    top_color * t + bottom_color * (1.0 - t)
}

fn uv_mapping(intersect: &Intersect) -> (f32, f32) {
    let u = 0.5 + (intersect.normal.x.atan2(intersect.normal.z)) / (2.0 * PI);
    let v = 0.5 - (intersect.normal.y.asin()) / PI;
    (u, v)
}

fn render(framebuffer: &mut Framebuffer, objects: &[Box<dyn RayIntersect>], camera: &Camera, light: &Light, ambient_light: f32, time: f32) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 4.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let rotated_direction = camera.base_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, ambient_light, 0, time);
            framebuffer.set_current_color(pixel_color.to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Raytracer with Animated Textures",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let body_material = Material::new(
        Color::new(100, 100, 255), // Color azul para el cuerpo
        30.0,
        [0.6, 0.3, 0.0, 0.0],
        0.0,
        "cuerpo"
    );
    
    let head_material = Material::new(
        Color::new(200, 50, 50), // Color rojo para la cabeza
        50.0,
        [0.4, 0.4, 0.2, 0.0],
        0.0,
        "cabeza"
    );
    
    let leg_material = Material::new(
        Color::new(80, 80, 80), // Color gris para las piernas
        20.0,
        [0.8, 0.2, 0.0, 0.0],
        0.0,
        "piernas"
    );
    
    let arm_material = Material::new(
        Color::new(80, 100, 80), // Verde oscuro para los brazos
        10.0,
        [0.6, 0.3, 0.0, 0.0],
        0.0,
        "brazos"
    );
    
    let ground_material = Material::new(
        Color::new(34, 139, 34), // Color verde para el suelo (césped)
        10.0,
        [0.6, 0.2, 0.0, 0.0],
        0.0,
        "suelo"
    );
    
    let sun_material = Material::new(
        Color::new(255, 223, 0), // Color amarillo para el sol
        50.0,
        [1.0, 0.5, 0.0, 0.0],
        0.0,
        "sol"
    );
    

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 1.0, 0.0), 
            radius: 0.5,
            material: head_material.clone(),
        }),
        Box::new(Cube {
            min: Vec3::new(-0.5, -1.0, -0.5),
            max: Vec3::new(0.5, 0.5, 0.5),
            material: body_material.clone(),
        }),
        Box::new(Cube {
            min: Vec3::new(-0.3, -2.0, -0.3),
            max: Vec3::new(-0.1, -1.0, 0.1),
            material: leg_material.clone(),
        }),
        Box::new(Cube {
            min: Vec3::new(0.1, -2.0, -0.3),
            max: Vec3::new(0.3, -1.0, 0.1),
            material: leg_material.clone(),
        }),
        Box::new(Cube {
            min: Vec3::new(-1.0, 0.0, -0.3),
            max: Vec3::new(-0.7, 0.5, 0.3),
            material: arm_material.clone(),
        }),
        Box::new(Cube {
            min: Vec3::new(0.7, 0.0, -0.3),
            max: Vec3::new(1.0, 0.5, 0.3),
            material: arm_material.clone(),
        }),
        Box::new(Cube {
            min: Vec3::new(-10.0, -2.1, -10.0),
            max: Vec3::new(10.0, -2.0, 10.0),
            material: ground_material.clone(),
        }),
        Box::new(Sphere {
            center: Vec3::new(5.0, 5.0, -5.0), 
            radius: 1.0,
            material: sun_material.clone(),
        }),
    ];

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 10.0), 
        Vec3::new(0.0, 0.0, 0.0),  
        Vec3::new(0.0, 1.0, 0.0),  
    );

    let mut light = Light::new(Vec3::new(1.0, -1.0, 5.0), Color::new(255, 255, 255), 1.0);
    let start_time = Instant::now();
    let rotation_speed = PI / 10.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed_time = start_time.elapsed().as_secs_f32();
        let time_factor = (elapsed_time % DAY_DURATION) / DAY_DURATION;

        let light_angle = time_factor * 2.0 * PI;
        light.position.x = light_angle.cos() * 10.0;
        light.position.y = light_angle.sin() * 10.0;

        if time_factor < 0.25 {
            light.color = Color::new(255, 223, 186);  
            light.intensity = 1.2;  
        } else if time_factor < 0.75 {
            light.color = Color::new(255, 255, 224);
            light.intensity = 1.5;  
        } else {
            light.color = Color::new(255, 140, 0);
            light.intensity = 1.0;  
        }

        let ambient_light = if time_factor < 0.5 {
            AMBIENT_LIGHT_DAY
        } else {
            AMBIENT_LIGHT_NIGHT
        };

        if window.is_key_down(Key::W) {
            camera.eye += camera.direction() * 0.1;  
        }
        if window.is_key_down(Key::S) {
            camera.eye -= camera.direction() * 0.1;  
        }

        if window.is_key_down(Key::Left) {
            camera.orbit(rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-rotation_speed, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, -rotation_speed);
        }

        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, rotation_speed);
        }

        render(&mut framebuffer, &objects, &camera, &light, ambient_light, elapsed_time);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
