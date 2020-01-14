use crate::math::Vec3;

trait EngineObject {
    fn update(&self);
    fn draw(&self);
}

struct SpaceDust {
    pos: Vec3
}
impl EngineObject for SpaceDust {
    fn update(&self) {

    }
    fn draw(&self) {

    }
}

pub struct Engine {
    a: i32
    // objects
    // camera
    // position
}

impl Engine {
    pub fn new() -> Engine {
        Engine { a: 0 }
    }

    pub fn render(&self) {
        // for each object .draw()
    }

    pub fn eval(&self) {
        // time
        // for each object .update()
    }
}
