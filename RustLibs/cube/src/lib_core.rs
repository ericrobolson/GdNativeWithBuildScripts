/// Cardinal directions. Limit characters/npcs to these directions.
#[derive(Copy, Clone, Debug, PartialEq)]
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

impl Direction {
    fn to_normalized_vec3d(&self) -> Vec3d {
        unimplemented!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FixedNumber {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Range {
    pub value: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3d {
    pub x: i32, //TODO: fixed point conversion
    pub y: i32, //TODO: fixed point conversion
    pub z: i32, //TODO: fixed point conversion
}

impl std::ops::AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Vec3d) {
        todo!()
    }
}

impl Range {
    pub const max: u8 = 255;
    pub const min: u8 = 0;

    pub fn map<T>(value: T, min: T, max: T) -> Self {
        unimplemented!();

        let v = Self::min;

        return Self { value: v };
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputType {
    Pressed,
    Released,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EngineInputs {
    MoveLeft(InputType),
    MoveRight(InputType),
    MoveUp(InputType),
    MoveDown(InputType),
    HeavyAttack(InputType),
    LightAttack(InputType),
    Block(InputType),
    Dodge(InputType),
}
