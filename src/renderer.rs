use sdl2::{pixels::Color, rect::Point, render::WindowCanvas, video::Window};

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_circle(&mut self, origin: Point, diameter: i32) -> Result<(), String> {
        let radius = diameter / 2;
        let center = Point::new(origin.x + radius, origin.y + radius);
        self.canvas.draw_point(center)?;
        // iterate row-wise
        let radius_squared = radius.pow(2);
        for row in origin.x..diameter {
            for col in origin.y..diameter {
                let distance_squared = (center.x - row).pow(2) + (center.y - col).pow(2);
                if distance_squared <= radius_squared {
                    let fill_point = Point::new(row, col);
                    self.canvas.draw_point(fill_point)?;
                }
            }
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        let point = Point::new(0, 0);
        self.draw_circle(point, 100)?;

        self.canvas.present();

        Ok(())
    }
}
