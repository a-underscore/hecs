use glium::Display;

pub struct Scene {
    pub display: Display,
    pub bg: [f32; 4],
}

impl Scene {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self { display, bg }
    }
}
