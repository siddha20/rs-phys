use super::entity::Entity;

#[derive(Debug)]
pub struct World {
    ents: Vec<Entity>
}


impl World {
    pub fn new() -> Self {
        World {
            ents: Vec::new()
        }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.ents.push(entity);
    }
}