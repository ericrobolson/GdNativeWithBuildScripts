use crate::ecs::{components, World};

use crate::lib_core::{Direction, EngineInputs, FixedNumber, Vec3d};

/// This system applies character actions from inputs
pub fn character_action_system(world: &mut World) {
    for e in world.entities() {
        let velocity = &world.velocities[e];
        let engine_inputs = &world.engine_inputs[e];
        let move_speed = &world.move_speeds[e];

        if velocity.is_none() || engine_inputs.is_none() || move_speed.is_none() {
            continue;
        }

        let move_speed = move_speed.as_ref().unwrap();
        let engine_inputs = engine_inputs.as_ref().unwrap();

        let mut movement_vec = Vec3d::default();

        for input in &engine_inputs.inputs {
            match input {
                EngineInputs::MoveDown(_) => {
                    movement_vec += Direction::South.to_normalized_vec3d();
                }
                EngineInputs::MoveUp(_) => {
                    movement_vec += Direction::North.to_normalized_vec3d();
                }
                EngineInputs::MoveLeft(_) => {
                    movement_vec += Direction::West.to_normalized_vec3d();
                }
                EngineInputs::MoveRight(_) => {
                    movement_vec += Direction::East.to_normalized_vec3d();
                }
                _ => {
                    // Ignore anything other than inputs
                }
            }
        }

        let normalized_movement_vec = movement_vec.normalize();
        let velocity_vec = normalized_movement_vec.multiply(move_speed.value);

        let mut velocity = velocity.clone().unwrap();
        velocity.value = velocity_vec;

        world.velocities[e] = Some(velocity);

        world.facing_direction[e] = Some(components::FacingComponent::new(Direction::from_vec3d(
            &velocity_vec,
        )));
    }
}

/// This system cleans up any input, leaving a blank slate for the next run.
pub fn input_cleanup_system(world: &mut World) {
    for e in world.entities() {
        let engine_inputs = world.engine_inputs[e].clone();

        if engine_inputs.is_none() {
            continue;
        }

        let mut engine_inputs = engine_inputs.unwrap();

        engine_inputs.inputs = vec![];

        world.engine_inputs[e] = Some(engine_inputs);
    }
}

/// Apply velocities to the positions
pub fn position_update_system(world: &mut World) {
    for e in world.entities() {
        let velocity = world.velocities[e].clone();
        let transform = world.transforms[e].clone();

        if velocity.is_none() || transform.is_none() {
            continue;
        }

        let velocity = velocity.as_ref().unwrap();
        let mut transform = transform.clone().unwrap();

        transform.position += velocity.value;
        world.transforms[e] = Some(transform);
    }
}
