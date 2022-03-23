use glam::DVec2;
//use nvm_sdf::flat::{Shape, ShapeSDF, ops_raw::angle_to_vec};

pub struct World {
    //root: Shape,
    size: (u32, u32),
    dt: f64,
}

impl World {
    pub fn new(w: u32, h: u32) -> Self {
        Self{
            /*root: Shape::union(Box::new([
                Shape::rectangle(DVec2::new(128.0, 64.0)).with_transform(
                    DVec2::new(256.0, 256.0),
                    22.5_f64.to_radians(),
                ),
                Shape::rectangle(DVec2::new(232.0, 256.0)).with_transform(
                    DVec2::new(116.0, 128.0),
                    45.0_f64.to_radians(),
                )
            ])),*/
            size: (w, h).to_owned(),
            dt: 0.0,
        }
    }

    pub fn set_size(&mut self, w: u32, h: u32) {
        self.size = (w, h);
    }

    pub fn update(&mut self) {
        self.dt += 1.0/60.0;

        /*let mut rot = std::f64::consts::PI;
        if let Shape::OpCombination(root) = &mut self.root {
            for child in root.children_mut() {
                child.set_origin(child.get_origin() + angle_to_vec(self.dt + rot));
                rot = -rot.signum() * (rot.abs() + std::f64::consts::PI/2.0)
            }
        }*/
    }

    pub fn draw(&self, frame: &mut [u8]) {
        /*for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.size.0 as usize) as f64;
            let y = (i / self.size.0 as usize) as f64;
            let rgba = Self::process(&self.root, x, y);
            pixel.copy_from_slice(&rgba);
        }*/
    }
}

impl World {
    /*fn process(shape: &Shape, x: f64, y: f64) -> [u8; 4] {
        let dist = shape.signed_distance(DVec2::new(x, y));
        Self::get_debug_rgb(dist, 160, 192, 255)
    }*/

    fn get_debug_rgb(dist: f64, pos: u8, neg: u8, bound: u8) -> [u8; 4] {
        let is_coloured = Self::gen_pattern_ridges(2.0, 4.0, 24.0, 4, dist);
        [
            if dist < 0.0 && is_coloured { neg   } else { 0 },
            if dist > 0.0 && is_coloured { pos   } else { 0 },
            if dist.abs() <= 1.0         { bound } else { 0 },
            255
        ]
    }

    fn gen_pattern_ridges(
        line_thin:  f64, 
        line_thick: f64,
        subsection_size: f64, 
        subsection_count: usize,
        dist: f64, 
    ) -> bool {
        let dist = dist.abs();
        let sub  = (dist.rem_euclid(subsection_size * subsection_count as f64) / subsection_size).round() as usize;
        let dist = dist.rem_euclid(subsection_size);
        let line = if sub == 0 || sub == subsection_count { line_thick } else { line_thin };
        !(dist < line || dist > subsection_size - line)
    }
}


