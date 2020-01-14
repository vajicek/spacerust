use std::time::SystemTime;

use crate::math::Vec3;

trait EngineObject {
    fn update(&self, engine_context: &Engine);
    fn draw(&self);
}

struct SpaceDust {
    pos: Vec3,
    vel: Vec3,
    last_update: SystemTime
}
impl SpaceDust {
    pub fn new (pos: Vec3, vel: Vec3) -> SpaceDust {
        SpaceDust {
            pos: pos,
            vel: vel,
            last_update: SystemTime::now()
        }
    }
}
impl EngineObject for SpaceDust {
    fn update(&self, engine_context: &Engine) {

    }
    fn draw(&self) {

    }
}

fn generate_space_dust(mut engine_objects: &Vec<Box<dyn EngineObject>>, count: i32) {
    for n in 1..101 {
        engine_objects.push(Box::new(SpaceDust::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0))));
    }
}

pub struct Engine {
    pub engine_objects: Vec<Box<dyn EngineObject>>,
    // objects
    // camera
    // position
}

impl Engine {
    pub fn new() -> Engine {
        let mut engine_objects: Vec::<Box<dyn EngineObject>> = Vec::with_capacity(10);
        generate_space_dust(&engine_objects, 100);
        Engine { engine_objects: engine_objects }
    }

    pub fn render(&self) {
        // for each object .draw()
    }

    pub fn eval(&self) {
        // time
        // for each object .update()
    }
}
