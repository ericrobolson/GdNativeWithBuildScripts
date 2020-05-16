//TODO: come up with some sort of system for combos.

/// Cardinal directions. Limit characters/npcs to these directions.
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct HitPoints {
    pub value: u32,
    pub max_value: u32,
}

pub struct Character {
    pub hp: HitPoints,
    pub ailments: Vec<Ailment>,
}

pub struct Range {
    pub value: u8,
}

impl Range {
    pub const max: u8 = 255;
    pub const min: u8 = 0;
}

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

/// Information related to an ailment a character is inflicted with.
pub struct Ailment {
    /// The type of the modifier
    pub modifier_type: AilmentTypes,
    /// The multiplier of the modifier
    pub modifier_multiplier: u8,
    /// The current meter of the multiplier. Causes the effect to be triggered when full.
    pub status_meter: Range,
}
