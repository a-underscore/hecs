use super::{component_manager::ComponentManager, entity_manager::EntityManager};
use glium::Display;

pub struct World<'a> {
    pub em: EntityManager,
    pub cm: ComponentManager<'a>,
    pub display: Display,
    pub bg: [f32; 4],
}

impl<'a> World<'a> {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self {
            em: EntityManager::default(),
            cm: ComponentManager::default(),
            display,
            bg,
        }
    }
}
