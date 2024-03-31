mod ecs;
use ecs::{Transform, *};

use raylib::prelude::*;

fn main() {

    // initialise the ECS
    let mut world = World::new();
    let entity1 = world.add_entity();
    world.add_component(entity1, Component::Rectangle(Rect { w: 100.0, h: 50.0 }));
    world.add_component(entity1, Component::Health(Health(100.0)));

    let entity2 = world.add_entity();
    world.add_component(entity2, Component::Rectangle(Rect{ w:50.0, h: 100.0 }));
    world.add_component(entity2, Component::Transform(Transform{ 
        pos: [10.0, 10.0],
        rot: [10.0, 10.0],
        scale: [10.0, 10.0],
     }));

    // initialise RayLib
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Game")
        // .vsync()
        .build();

    while !rl.window_should_close() {

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("Hello World with RayLib in Rust!", 10, 10, 24, Color::RAYWHITE);
        
        d.draw_fps(100, 100);

        // run Systems
        world.run_systems(&mut d);
    }
}