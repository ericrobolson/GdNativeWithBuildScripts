mod fixed_number;
pub use fixed_number::FixedNumber;

mod vec3d;
pub use vec3d::Vec3d;

mod range;
pub use range::Range;

mod direction;
pub use direction::Direction;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputType {
    Pressed,
    Held,
    Released,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EngineInputs {
    MoveLeft(InputType),
    MoveRight(InputType),
    MoveUp(InputType),
    MoveDown(InputType),
    HeavyAttack(InputType),
    MediumAttack(InputType),
    LightAttack(InputType),
    Jump(InputType),
}
