use super::entity::Entity;
use super::physics::{
    fv,
    fa,
    rk4
};

#[derive(Debug)]
pub struct World {
    ents: Vec<Entity>,
    time: f64,
    time_step: f64
}


impl World {
    pub fn new() -> Self {
        World {
            ents: Vec::new(),
            time: 0.0,
            time_step: 0.0001
        }
    }

    pub fn set_time_step(&mut self, t: f64) {
        self.time_step = t;
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.ents.push(entity);
    }

    pub fn get_ents(&self) -> &Vec<Entity> {
        &self.ents
    }

    pub fn update(&mut self) {
        for e in &mut self.ents {
            let  (_, x, v) = rk4(self.time_step, 
                                          self.time, 
                                          e.pos[0], 
                                          e.vel[0], 
                                          fv, fa);
            e.pos[0] = x;
            e.vel[0] = v;
        }


        self.time += self.time_step;
    }
}