use crate::ecs::World;

/// This system applies character actions from inputs
pub fn character_action_system(world: &mut World) {
    for e in 0..world.next_entity {
        let velocity = world.velocities[e];
        let engine_inputs = world.engine_inputs[e].clone();
        let move_speed = world.move_speeds[e];

        if velocity.is_none() || engine_inputs.is_none() || move_speed.is_none() {
            continue;
        }

        //TODO: apply character movements
    }
}

/// This system cleans up any input, leaving a blank slate for the next run.
pub fn input_cleanup_system(world: &mut World) {
    for e in 0..world.next_entity {
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
    for e in 0..world.next_entity {
        let velocity = world.velocities[e];
        let transform = world.transforms[e];

        if velocity.is_none() || transform.is_none() {
            continue;
        }

        let velocity = velocity.unwrap();
        let mut transform = transform.unwrap();

        transform.position += velocity.value;

        world.transforms[e] = Some(transform);
    }
}
