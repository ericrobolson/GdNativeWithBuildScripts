use crate::lib_core::{Direction, EngineInputs, FixedNumber, Range, Vec3d};

#[derive(Clone, Debug, PartialEq)]
pub enum Components {
    HitPoints(HitPointComponent),
    Player(PlayerComponent),
    Facing(FacingComponent),
    EngineInputs(EngineInputsComponent),
    Ailments(AilmentsComponent),
    Transform(TransformComponent),
    Velocity(VelocityComponent),
    MoveSpeedComponent(MoveSpeedComponent),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MoveSpeedComponent {
    pub value: FixedNumber,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HitPointComponent {
    pub value: u32,
    pub max_value: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VelocityComponent {
    pub value: Vec3d,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransformComponent {
    pub position: Vec3d,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlayerComponent {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FacingComponent {
    pub direction: Direction,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EngineInputsComponent {
    pub inputs: Vec<EngineInputs>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AilmentsComponent {
    pub ailments: Vec<Ailment>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Information related to an ailment a character is inflicted with.
pub struct Ailment {
    /// The type of the modifier
    pub modifier_type: AilmentTypes,
    /// The multiplier of the modifier
    pub modifier_multiplier: u8,
    /// The current meter of the multiplier. Causes the effect to be triggered when full.
    pub status_meter: Range,
}

#[derive(Copy, Clone, Debug, PartialEq)]
/// Various types of ailments that can be inflicted upon characters. For now, keep small until more things need to be added.
pub enum AilmentTypes {
    /// A modifier which when triggered, causes a single hp loss event and resets the meter to 0.
    Bleed,
    /// A modifier which when triggered, will cause small HP loss until meter is drained completely.
    Poison,
    /// A modifier which when triggered, will instantly kill the character.
    Curse,
    /// A modifier which when triggered, causes all attacks and movement to increase in time to execute.
    Sloth,
    /// A modifier which when triggered, doubles the meter of any current ailments.
    BrainWorms,
}

impl AilmentTypes {
    pub const size: usize = 5;

    pub fn index(&self) -> usize {
        let index = match self {
            AilmentTypes::Bleed => 0,
            AilmentTypes::Poison => 1,
            AilmentTypes::Curse => 2,
            AilmentTypes::Sloth => 3,
            AilmentTypes::BrainWorms => 4,
        };

        return index;
    }

    pub fn display_name(&self) -> String {
        let name = match self {
            AilmentTypes::Bleed => String::from("Bleed"),
            AilmentTypes::Poison => String::from("Poison"),
            AilmentTypes::Curse => String::from("Curse"),
            AilmentTypes::Sloth => String::from("Sloth"),
            AilmentTypes::BrainWorms => String::from("Brainworms"),
        };

        return name.to_uppercase();
    }

    pub fn description(&self) -> String {
        let description = match self {
            AilmentTypes::Bleed => String::from("causes a single loss of HP."),
            AilmentTypes::Poison => String::from("causes HP loss until meter runs out."),
            AilmentTypes::Curse => String::from("causes instant death."),
            AilmentTypes::Sloth => String::from("slows down character movement and attacks."),
            AilmentTypes::BrainWorms => String::from("doubles the meter of all ailments."),
        };

        return format!("{}: When triggered, {}", self.display_name(), description);
    }
}
