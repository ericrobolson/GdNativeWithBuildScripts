pub mod systems;

pub mod components;
use components::{
    AilmentsComponent, Components, EngineInputsComponent, FacingComponent, HitPointComponent,
    MoveSpeedComponent, PlayerComponent, TransformComponent, VelocityComponent,
};

pub type Entity = usize;

pub type Storage<T> = Vec<Option<T>>;

// TODO: parent/child implementation based off of this:
// http://bitsquid.blogspot.com/2014/10/building-data-oriented-entity-system.html
pub struct World {
    pub next_entity: Entity,
    pub parents: Storage<Entity>,
    pub ailments: Storage<AilmentsComponent>,
    pub engine_inputs: Storage<EngineInputsComponent>,
    pub facing_direction: Storage<FacingComponent>,
    pub hitpoints: Storage<HitPointComponent>,
    pub players: Storage<PlayerComponent>,
    pub transforms: Storage<TransformComponent>,
    pub velocities: Storage<VelocityComponent>,
    pub move_speeds: Storage<MoveSpeedComponent>,
}

impl World {
    pub const MAX_ENTITIES: usize = 1000;

    pub fn new() -> Self {
        return Self {
            next_entity: 0,
            parents: generate_storage(),
            ailments: generate_storage(),
            engine_inputs: generate_storage(),
            facing_direction: generate_storage(),
            hitpoints: generate_storage(),
            players: generate_storage(),
            transforms: generate_storage(),
            velocities: generate_storage(),
            move_speeds: generate_storage(),
        };
    }

    pub fn dispatch(&mut self) {
        systems::character_action_system(self);
        systems::input_cleanup_system(self);
        systems::position_update_system(self);

        self.maintain();
    }

    pub fn add_entity(&mut self) -> Entity {
        let e = self.next_entity;

        self.next_entity += 1;

        return e;
    }

    pub fn register_component(&mut self, entity: Entity, component: Components) {
        match component {
            Components::HitPoints(c) => self.hitpoints[entity] = Some(c),
            Components::Player(c) => self.players[entity] = Some(c),
            Components::Facing(c) => self.facing_direction[entity] = Some(c),
            Components::EngineInputs(c) => self.engine_inputs[entity] = Some(c),
            Components::Ailments(c) => self.ailments[entity] = Some(c),
            Components::Transform(c) => self.transforms[entity] = Some(c),
            Components::Velocity(c) => self.velocities[entity] = Some(c),
            Components::MoveSpeedComponent(c) => self.move_speeds[entity] = Some(c),
        };
    }

    fn maintain(&mut self) {
        //TODO: shift entities over, delete any missing entities, update indexes.
    }

    fn delete_entity(&mut self, entity: Entity) {
        self.hitpoints[entity] = None;
        self.players[entity] = None;
        self.facing_direction[entity] = None;
        self.engine_inputs[entity] = None;
        self.ailments[entity] = None;
        self.transforms[entity] = None;
        self.velocities[entity] = None;
        self.move_speeds[entity] = None;
    }
}

fn generate_storage<TComponent>() -> Storage<TComponent> {
    let mut v = Vec::<Option<TComponent>>::with_capacity(World::MAX_ENTITIES);

    for _ in 0..World::MAX_ENTITIES {
        v.push(None);
    }

    return v;
}
