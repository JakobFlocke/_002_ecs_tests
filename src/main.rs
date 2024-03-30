use std::collections::HashMap;

fn main() {

    let mut world = World::new();

    world.add_entity();
    world.add_entity();

    world.append_component((Component::Transform, Entity(0)));
    world.append_component((Component::Health, Entity(0)));
    world.append_component((Component::Transform, Entity(1)));
    
    loop {
        for ent in &(world.entities) {
            
            // if the entity has transform component move, run the system
            if let Some(&comp_id) = world.entity_transform_relation.get(&ent.0) {
                // if let Some(transform) = world.transform_components.get_mut(comp_id as usize) {
                //     move_system(transform);
                // }
                move_system(world.transform_components.get_mut(comp_id as usize).unwrap());
            }

            // if the entity has transform component move, run the system
            if let Some(&comp_id) = world.entity_health_relation.get(&ent.0) {
                take_damage(world.health_components.get_mut(comp_id as usize).unwrap())
            }
        }
    }

}

enum Component {
    Transform,
    Health,
}

/* Entity */
#[derive(Copy, Clone, Debug)] // Derive Copy and Clone traits for Entity
struct Entity(u32);

/* World */
struct World {
    entities: Vec<Entity>,
    
    /* Component Data */
    health_components: Vec<Health>,
    transform_components: Vec<Transform>,

    /* Component Relations */
    entity_health_relation: HashMap<u32, u32>,
    entity_transform_relation: HashMap<u32, u32>,
}

impl World {
    fn new() -> Self {
        Self {
            entities: Vec::new(),
            health_components: Vec::new(),
            transform_components: Vec::new(),
            entity_health_relation: HashMap::new(),
            entity_transform_relation: HashMap::new(),
        }
    }

    fn add_entity(&mut self) {
        self.entities.push(Entity(self.entities.len() as u32));
    }

    fn append_component(&mut self, pair: (Component, Entity)) {
        match pair.0 {
            
            Component::Health => {
                // Append health component
                self.health_components.push(Health(100.0));
                // Update entity-health relation
                self.entity_health_relation.insert(self.health_components.len() as u32, pair.1.0);
            }
            Component::Transform => {
                self.transform_components.push(Transform {
                    pos: [10.0, 10.0],
                    rot: [10.0, 10.0],
                    scale: [10.0, 10.0],
                });
                self.entity_transform_relation.insert(self.transform_components.len() as u32, pair.1.0);
            }
        }
    }
    
}


/* Components */
#[derive(Debug)]
struct Health(f32);

#[derive(Debug)]
struct Transform { pos: [f32; 2], rot: [f32; 2], scale: [f32; 2] }

/* Systems */
fn move_system(tf: &mut Transform) {
    // movement logic (z.B. pos = dir * speed)
}

fn take_damage(health: &mut Health) {
    // health logic
    if health.0 == 0.0 {
        println!("Died!")
    }
}