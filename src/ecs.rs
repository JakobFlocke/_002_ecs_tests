use std::collections::{HashMap, HashSet};
use raylib::prelude::*;

/* Entity */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(pub u32);

/* Component Data */
#[derive(Debug)]
pub struct Health(pub f32);

#[derive(Debug)]
pub struct Transform {
    pub pos: [f32; 2],
    pub rot: [f32; 2],
    pub scale: [f32; 2],
}

#[derive(Debug)]
pub struct Rect {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug)]
pub struct Sprite(pub Texture2D);

/* Systems */
pub fn move_system(transform: &mut Transform) {
    // Movement logic
}

pub fn render_system(components: &Vec<Component>, d: &mut RaylibDrawHandle<'_>) {
    for component in components {
        match component {
            Component::Transform(_) => {} // Handle transform rendering if needed
            Component::Health(_) => {} // Handle health rendering if needed
            Component::Rectangle(rectangle) => {
                // Rendering logic for rectangle
                d.draw_rectangle(300, 300, rectangle.w as i32, rectangle.h as i32, Color::WHITE);
            }
            Component::Sprite(sprite) => {
                // Rendering logic for sprite
                println!("Rendering sprite: {:?}", sprite);
            }
        }
    }
}

/* World */
pub struct World {
    entities: HashMap<Entity, Vec<Component>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        let entity = Entity(self.entities.len() as u32);
        self.entities.insert(entity, Vec::new());
        entity
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        if let Some(components) = self.entities.get_mut(&entity) {
            components.push(component);
        } else {
            self.entities.insert(entity, vec![component]);
        }
    }

    pub fn run_systems(&mut self, d: &mut RaylibDrawHandle<'_>) {
        for (entity, components) in &mut self.entities {
            render_system(components, d);
        }
    }
}

/* Components */
pub enum Component {
    Transform(Transform),
    Health(Health),
    Rectangle(Rect),
    Sprite(Sprite),
}
