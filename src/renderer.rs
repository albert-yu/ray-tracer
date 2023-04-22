use sdl2::{pixels::Color, rect::Point, render::WindowCanvas, video::Window};

mod scene;
use scene::{distance_squared, Point3D, Sphere};

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    // fn draw_circle(&mut self, origin: Point, diameter: i32) -> Result<(), String> {
    //     let radius = diameter / 2;
    //     let center = Point::new(origin.x + radius, origin.y + radius);
    //     self.canvas.draw_point(center)?;
    //     // iterate row-wise
    //     let radius_squared = radius.pow(2);
    //     for row in origin.x..diameter {
    //         for col in origin.y..diameter {
    //             let distance_squared = (center.x - row).pow(2) + (center.y - col).pow(2);
    //             if distance_squared <= radius_squared {
    //                 let fill_point = Point::new(row, col);
    //                 self.canvas.draw_point(fill_point)?;
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    // fn draw_scaled_point(&mut self, point: Point, scale_factor: i32) {}

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

        let scale_factor = (canvas_w as i32) / X_WIDTH;

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
                        let draw_x = origin_x + x * scale_factor;
                        let draw_y = origin_y + y * scale_factor;

                        self.canvas.draw_point(Point::new(draw_x, draw_y))?;
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
