use raylib::prelude::*;

use std::collections::HashMap;

pub struct World {
    // Store components for each entity
    entities: Vec<HashMap<ComponentType, Component>>,
    // Store identifiers for each entity
    entity_ids: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            entity_ids: Vec::new(),
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        // Generate a new entity ID and push an empty component vector
        let entity = Entity(self.entity_ids.len() as u32);
        self.entity_ids.push(entity);
        self.entities.push(HashMap::new());
        entity
    }

    pub fn add_component(&mut self, entity: Entity, component_type: ComponentType, component: Component) {
        // Add a component to the corresponding entity
        let index = entity.0 as usize;
        if index < self.entities.len() {
            self.entities[index].insert(component_type, component);
        }
    }
    
    

    pub fn run_systems(&mut self, d: &mut RaylibDrawHandle<'_>) {

        for mut components in &mut self.entities {
            if let Some(movement_component) = components.get_mut(&ComponentType::Movement) {
                movement_system(&mut components);
            }
            
            if components.contains_key(&ComponentType::Shape) || components.contains_key(&ComponentType::Sprite) {
                // Run the render system for entities with Shape or Sprite components
                render_system(components, d);
            }
        }
        
    }
}



/* Entity */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(pub u32);

/* Components */
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ComponentType {
    Transform,
    Shape,
    Sprite,
    Movement,
    Stats,
}

#[derive(Debug)]
pub enum Component {
    Transform(TransformCmp),
    Shape(ShapeCmp),
    Sprite(SpriteCmp),
    Movement(MovementCmp),
    Stats(StatCmp),
}

/* Component Data */
#[derive(Debug)]
pub struct TransformCmp {
    pub pos: [f32; 2],
    pub rot: [f32; 2],
    pub scale: [f32; 2],
}

#[derive(Debug)]
pub enum ShapeCmp {
    Square(u32, Color),
    Rectangle(u32, u32, Color),
    Circle(f32, Color),
}

#[derive(Debug)]
pub struct SpriteCmp(pub Texture2D);

#[derive(Debug)]
pub struct MovementCmp {
    pub dir: [f32; 2],
}

#[derive(Debug)]
pub enum Stat<T> {
    VITALITY(T),       // health
    ENDURANCE(T),      // stamina
    WISDOM(T),         // mana
    SPEED(T),          // speed, agility, ...
    STRENGTH(T),       // strength, mele damage, ...
    TOUGHNESS(T),      // protection
    PERSEPTION(T),     // perseprion (look range, better identify, ...)
    INTELLIGENCE(T),   // mind magic, ...
}

#[derive(Debug)]
pub struct StatCmp { // 164 Byte
    pub base: [Stat<u32>; 8],   // base stats
    pub mult: [Stat<f32>; 8],   // stat multipliers
    pub add: [Stat<u32>; 8],    // stat boni
    pub fin: [Stat<u32>; 8],    // final stats
    pub max_status: [u32;3],    // maximum satatus (hp, sp, mp)
    pub curr_status: [f64;3],   // current status (hp, sp, mp)
}

#[derive(Debug)]
pub enum Race {
    NONE,
    HUMAN,
    ELF,
    DWARF,
}

#[derive(Debug)]
pub struct RaceCmp {
    pub level: u32,
    pub xp: f64,
    pub race: Race,
}

pub struct ClassCmp {}

pub struct ProfessionCmp {}

/* Systems (temporary solutions) */

pub fn render_system(components: &mut HashMap<ComponentType, Component>, d: &mut RaylibDrawHandle<'_>) {
    // Render system for Shape component
    if let Some(Component::Shape(shape)) = components.get(&ComponentType::Shape) {
        // Check if there is a Transform component
        if let Some(Component::Transform(tf)) = components.get(&ComponentType::Transform) {
            // Render the shape at the position specified by the Transform component
            match shape {
                ShapeCmp::Rectangle(width, height, color) => {
                    d.draw_rectangle(tf.pos[0] as i32, tf.pos[1] as i32, *width as i32, *height as i32, *color);
                }
                ShapeCmp::Circle(radius, color) => {
                    d.draw_circle(tf.pos[0] as i32, tf.pos[1] as i32, *radius, *color);
                }
                ShapeCmp::Square(size, color) => {
                    d.draw_rectangle(tf.pos[0] as i32, tf.pos[1] as i32, *size as i32, *size as i32, *color);
                }
            }
        } else {
            // Render the shape at a standard position 50, 50
            match shape {
                ShapeCmp::Rectangle(width, height, color) => {
                    d.draw_rectangle(50 as i32, 50 as i32, *width as i32, *height as i32, *color);
                }
                ShapeCmp::Circle(radius, color) => {
                    d.draw_circle(50 as i32, 50 as i32, *radius, *color);
                }
                ShapeCmp::Square(size, color) => {
                    d.draw_rectangle(50 as i32, 50 as i32, *size as i32, *size as i32, *color);
                }
            }
        }
    }
}


pub fn movement_system(components: &mut HashMap<ComponentType, Component>) {
    let mut speed: u32 = 1;

    // Initialize dir to some default value
    let mut dir: [f32; 2] = [0.0, 0.0];

    // Extract direction vector from Movement component
    if let Some(Component::Movement(mov)) = components.get(&ComponentType::Movement) {
        dir = mov.dir;

        // Calculate the magnitude of the dir vector
        let magnitude = (dir[0].powi(2) + dir[1].powi(2)).sqrt();

        // Set the length of the dir vector to 1
        if magnitude != 0.0 {
            dir[0] /= magnitude;
            dir[1] /= magnitude;
        }
    }

    // Extract speed from Stats component
    if let Some(Component::Stats(stats)) = components.get(&ComponentType::Stats) {
        if let Stat::SPEED(s) = &stats.fin[3] {
            speed = *s;
        }
    }

    // Apply movement using normalized dir vector and speed
    if let Some(Component::Transform(tf)) = components.get_mut(&ComponentType::Transform) {
        tf.pos[0] += dir[0] * (speed as f32) * 0.01; // Adjust position based on direction and speed
        tf.pos[1] += dir[1] * (speed as f32) * 0.01; // Adjust position based on direction and speed
    }
}

