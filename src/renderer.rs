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
            radius: 200.0,
        };

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
        let r = sphere_1.radius;

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
                let c = scene.camera.position;
                let d = c - screen_point;
                let square_completion = 4.0 * (c * d) * (c * d) - 4.0 * (d * d) * (c * c - r);
                if square_completion >= 0.0 {
                    self.draw_scaled_point(
                        PointFloat {
                            x: x as f32,
                            y: y as f32,
                        },
                        scale_factor,
                    )?
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
