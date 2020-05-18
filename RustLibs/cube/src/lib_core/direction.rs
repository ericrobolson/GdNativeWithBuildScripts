use super::*;

/// Cardinal/intercardinal directions. Limit characters/npcs to these directions.
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
    pub fn to_normalized_vec3d(&self) -> Vec3d {
        unimplemented!();
    }

    pub fn from_vec3d(vector: &Vec3d) -> Self {
        unimplemented!();
    }
}
