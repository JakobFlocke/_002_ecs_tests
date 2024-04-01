mod ecs;
use ecs::{TransformCmp, *};

use raylib::prelude::*;

fn main() {

    // initialise the ECS
    let mut world = World::new();

    
    
    let entity = world.add_entity();
    world.add_component(entity, ComponentType::Shape, Component::Shape(ShapeCmp::Square(100, Color::WHITE)));
    world.add_component(entity, ComponentType::Transform, Component::Transform(TransformCmp{ 
        pos: [10.0, 10.0], // Bottom-left corner
        rot: [0.0, 0.0],
        scale: [10.0, 10.0],
    }));
    world.add_component(entity, ComponentType::Movement, Component::Movement(MovementCmp{dir:[1.0, 1.0]})); // Move to bottom left
    

    world.add_component(Entity(0), ComponentType::Input, Component::Input(InputCmp{mask:MOVEMENT_INPUT}));

    // Initialise RayLib
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Game")
        .build();

    while !rl.window_should_close() {
        // ECS update
        world.run_systems(&mut rl, &thread);
    }
}
