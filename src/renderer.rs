use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

mod scene;
use scene::{Sphere, Vec3};

use crate::renderer::scene::{Camera, Scene};

pub struct Renderer {
    canvas: WindowCanvas,
}

struct PointFloat {
    pub x: f32,
    pub y: f32,
}

struct RGBFloat {
    r: f32,
    g: f32,
    b: f32,
}

impl RGBFloat {
    /// Scalar is usually between 0 and 1
    pub fn scale(&self, scalar: f32) -> Self {
        RGBFloat {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }

    pub fn to_color(&self) -> Color {
        Color {
            r: self.r as u8,
            g: self.g as u8,
            b: self.b as u8,
            a: 0xff,
        }
    }
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_scaled_point(&mut self, point: PointFloat, scale_factor: f32) -> Result<(), String> {
        let x = point.x;
        let y = point.y;
        let scale_rounded = scale_factor.round() as u32;
        let draw_x = (x * scale_factor).round() as i32;
        let draw_y = (y * scale_factor).round() as i32;

        self.canvas
            .fill_rect(Rect::new(draw_x, draw_y, scale_rounded, scale_rounded))?;
        Ok(())
    }

    fn draw_scene(&mut self) -> Result<(), String> {
        let sphere_1 = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 5.0,
        };

        let sphere_2 = Sphere {
            center: Vec3 {
                x: -10.0,
                y: 10.0,
                z: 15.0,
            },
            radius: 5.0,
        };

        let spheres = [&sphere_1, &sphere_2];
        let colors = [
            RGBFloat {
                r: 0.0,
                g: 255.0,
                b: 0.0,
            },
            RGBFloat {
                r: 0.0,
                g: 255.0,
                b: 255.0,
            },
        ];

        let camera = Camera {
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -20.0,
            },
            up: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            right: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let scene = Scene {
            light: Vec3 {
                x: 0.0,
                y: 100.0,
                z: -200.0,
            },
            camera,
            focal_distance: 10.0,
            screen_width: 64,
            screen_height: 48,
        };

        let (canvas_w, _canvas_h) = self.canvas.output_size()?;

        let scale_factor = (canvas_w as f32) / scene.screen_width as f32;
        let screen_z = scene.camera.position.z + scene.focal_distance;

        let center_x = scene.screen_width / 2;
        let center_y = scene.screen_height / 2;

        for x in 0..scene.screen_width {
            for y in 0..scene.screen_height {
                let x_offset = x - center_x;
                let y_offset = center_y - y;
                let x_float = x_offset as f32;
                let y_float = y_offset as f32;
                let screen_point = Vec3 {
                    x: x_float,
                    y: y_float,
                    z: screen_z,
                };
                let o = scene.camera.position;
                let d = o - screen_point;
                let mut found_sphere_index: Option<usize> = None;
                let mut min_t: Option<f32> = None;
                for (index, sphere) in spheres.iter().enumerate() {
                    let r = sphere.radius;
                    let c = sphere.center;
                    let to_center = o - c;
                    let b = 2.0 * (to_center * d);
                    let a = d * d;
                    let c = to_center * to_center - r * r;
                    // b^2 - 4ac
                    let square_completion = b * b - 4.0 * a * c;
                    if square_completion >= 0.0 {
                        // compute parameter t
                        let square_root = square_completion.sqrt();
                        let t_plus = (-b + square_root) / (2.0 * a);
                        let t_minus = (-b - square_root) / (2.0 * a);
                        let ray_plus = d.scale(t_plus);
                        let ray_minus = d.scale(t_minus);
                        // pick closer
                        let dist_plus = ray_plus * ray_plus;
                        let dist_minus = ray_minus * ray_minus;
                        let t = if dist_minus < dist_plus {
                            t_minus
                        } else {
                            t_plus
                        };
                        match min_t {
                            Some(value) => {
                                if t < value {
                                    min_t = Some(t);
                                    found_sphere_index = Some(index);
                                }
                            }
                            None => {
                                min_t = Some(t);
                                found_sphere_index = Some(index);
                            }
                        }
                    }
                }
                match found_sphere_index {
                    Some(index) => match min_t {
                        Some(t) => {
                            let rgb = &colors[index];
                            let sphere = spheres[index];
                            let collision_point = scene.camera.position + d.scale(t);
                            let light_to_collision = scene.light - collision_point;
                            // using a nice property of spheres--any vector from surface to center
                            // is orthogonal to tangent plane

                            let normal = collision_point - sphere.center;
                            let unit_normal = normal.scale(1.0 / (normal * normal).sqrt());
                            let collision_norm = light_to_collision
                                .scale(1.0 / (light_to_collision * light_to_collision).sqrt());

                            let color_intensity_factor = unit_normal * collision_norm;

                            let adjusted_rgb = rgb.scale(color_intensity_factor);
                            let color = adjusted_rgb.to_color();
                            self.canvas.set_draw_color(color);
                            self.draw_scaled_point(
                                PointFloat {
                                    x: x as f32,
                                    y: y as f32,
                                },
                                scale_factor,
                            )?
                        }
                        None => {}
                    },
                    None => {}
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);

        self.draw_scene()?;
        self.canvas.present();

        Ok(())
    }
}
