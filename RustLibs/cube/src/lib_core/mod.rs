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
    MoveUp(InputType),
    MoveDown(InputType),
    MoveLeft(InputType),
    MoveRight(InputType),
    HorizontalLightAttack(InputType),
    HorizontalHeavyAttack(InputType),
    VerticalLightAttack(InputType),
    VerticalHeavyAttack(InputType),
}
