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
    Pressed(EngineInputs),
    Held(EngineInputs),
    Released(EngineInputs),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EngineInputs {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    HorizontalAttack,
    VerticalAttack,

    Jump,
    Dodge,
}
