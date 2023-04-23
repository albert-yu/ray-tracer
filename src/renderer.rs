use sdl2::{pixels::Color, rect::Point, render::WindowCanvas, video::Window};

mod scene;
use scene::{distance_squared, Point3D, Sphere};

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
        let rounded_x = x.round() as i32;
        let rounded_y = y.round() as i32;
        let scale_rounded = scale_factor.round() as i32;

        let pixels_to_draw = scale_rounded.pow(2);
        let mut points: Vec<Point> = Vec::with_capacity(pixels_to_draw as usize);
        points.push(Point::new(rounded_x, rounded_y));
        // expand to the right and down
        let mut y_offset = 0;
        for i in 1..pixels_to_draw {
            let x_offset = i % scale_rounded;
            if x_offset == 0 {
                y_offset += 1;
            }
            points.push(Point::new(rounded_x + x_offset, rounded_y + y_offset));
        }

        self.canvas.draw_points(points.as_slice())?;
        Ok(())
    }

    fn draw_scene(&mut self) -> Result<(), String> {
        const X_WIDTH: i32 = 300;
        const Y_HEIGHT: i32 = 200;
        const X_MIN: i32 = -150;
        const X_MAX: i32 = X_MIN + X_WIDTH;
        const Y_MIN: i32 = -100;
        const Y_MAX: i32 = Y_MIN + Y_HEIGHT;
        const Z_MIN: i32 = X_MIN;
        const Z_MAX: i32 = X_MAX;
        let spheres = [Sphere {
            center: Point3D { x: 0, y: 0, z: 0 },
            radius: 50,
        }];

        // let scene = Scene {
        //     camera: Camera {
        //         position: Point3D {
        //             x: 0,
        //             y: 0,
        //             z: Z_MAX,
        //         },
        //         target: Point3D { x: 0, y: 0, z: 0 },
        //     },
        // };

        let (canvas_w, canvas_h) = self.canvas.output_size()?;
        let origin_x = (canvas_w / 2) as i32;
        let origin_y = (canvas_h / 2) as i32;

        let scale_factor = (canvas_w as f32) / X_WIDTH as f32;

        for sphere in spheres {
            for x in X_MIN..X_MAX {
                for y in Y_MIN..Y_MAX {
                    for z in Z_MIN..Z_MAX {
                        let current_point = Point3D { x, y, z };
                        let dist_squared = distance_squared(&current_point, &sphere.center);
                        if dist_squared > sphere.radius.pow(2) {
                            continue;
                        }
                        // project 3D point onto 2D plane (TODO: do this correctly)
                        let draw_x = origin_x as f32 + x as f32 * scale_factor;
                        let draw_y = origin_y as f32 + y as f32 * scale_factor;

                        self.draw_scaled_point(
                            PointFloat {
                                x: draw_x,
                                y: draw_y,
                            },
                            scale_factor,
                        )?
                    }
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        // let point = Point::new(0, 0);
        // self.draw_circle(point, 100)?;

        self.draw_scene()?;
        self.canvas.present();

        Ok(())
    }
}
