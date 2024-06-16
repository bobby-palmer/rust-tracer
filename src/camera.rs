use crate::{vector::{self, Vector}, Unit};

pub struct Camera {
    pixel_height: u32,
    pixel_width: u32,
    upper_left: Vector,
    view_width: Vector,
    view_height: Vector
}

impl Camera {

    pub fn build(pixel_height: u32, aspect_ratio: Unit) -> Self {
        let pixel_width: u32 = (pixel_height as Unit * aspect_ratio) as u32;
        let width: Unit = 2.0;
        let height: Unit = width * pixel_height as Unit / pixel_width as Unit;
        let view_width = Vector::build(width, 0.0, 0.0);
        let view_height = Vector::build(0.0, -height, 0.0);
        let upper_left = Vector::build(0.0, 0.0, 1.0) - view_width.clone().scale(0.5) - view_height.clone().scale(0.5);
        Camera {
            pixel_width,
            pixel_height,
            upper_left,
            view_height,
            view_width
        }
    }

    pub fn draw(self) {
        println!("P3");
        println!("{} {}", self.pixel_width, self.pixel_height);
        println!("255");

        for row in 0..self.pixel_height {
            for col in 0..self.pixel_width {
                let width_fraction = col as Unit / self.pixel_width as Unit;
                let height_fraction = row as Unit / self.pixel_height as Unit;
            }
        }
    }
}
