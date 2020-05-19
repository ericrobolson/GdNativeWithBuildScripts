use crate::ecs::{components, World};
use crate::lib_core::FixedNumber;

pub fn assemblage_player(world: &mut World) {
    let e = world.add_entity();

    world.players[e] = Some(components::PlayerComponent::new());
    world.engine_inputs[e] = Some(components::EngineInputsComponent::new());
    world.transforms[e] = Some(components::TransformComponent::new());
    world.velocities[e] = Some(components::VelocityComponent::new());
    world.move_speeds[e] = Some(components::MoveSpeedComponent::new(FixedNumber::from_i32(
        2,
    )));
}
