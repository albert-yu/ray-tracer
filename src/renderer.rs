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
                y: 0.0,
                z: 15.0,
            },
            radius: 5.0,
        };

        let spheres = [&sphere_1, &sphere_2];
        let colors = [Color::GREEN, Color::CYAN];

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
                let y_offset = y - center_y;
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
                        let t = f32::min(t_plus, t_minus);
                        match min_t {
                            Some(value) => {
                                if t < value && t > 0.0 {
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
                    Some(value) => {
                        let color = colors[value];
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
