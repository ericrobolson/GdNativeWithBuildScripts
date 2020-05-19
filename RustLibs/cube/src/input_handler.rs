use super::*;
use lib_core::{EngineInputs, InputType};

struct InputConst {}
impl InputConst {
    pub const MOVE_UP: &'static str = "character_move_up";
    pub const MOVE_DOWN: &'static str = "character_move_down";
    pub const MOVE_LEFT: &'static str = "character_move_left";
    pub const MOVE_RIGHT: &'static str = "character_move_right";

    pub const JUMP: &'static str = "character_jump";
    pub const DODGE: &'static str = "character_dodge";

    pub const HORZ_ATK: &'static str = "character_horizontal_attack";
    pub const VERT_ATK: &'static str = "character_vertical_attack";

    fn inputs() -> Vec<(&'static str, EngineInputs)> {
        return vec![
            (Self::MOVE_UP, EngineInputs::MoveUp),
            (Self::MOVE_DOWN, EngineInputs::MoveDown),
            (Self::MOVE_LEFT, EngineInputs::MoveLeft),
            (Self::MOVE_RIGHT, EngineInputs::MoveRight),
        ];
    }
}

pub fn get_input_from_event(event: InputEvent) -> Option<InputType> {
    for (input_str, engine_input) in InputConst::inputs() {
        let i = input_from_event(input_str, engine_input, &event);
        if i.is_some() {
            return i;
        }
    }

    return None;
}

fn input_from_event(
    button: &'static str,
    input: EngineInputs,
    event: &InputEvent,
) -> Option<InputType> {
    if event.is_action_pressed(GodotString::from_str(button), false) {
        return Some(InputType::Pressed(input));
    } else if event.is_action_released(GodotString::from_str(button)) {
        return Some(InputType::Released(input));
    }

    return None;
}

pub fn input_poll() -> Vec<InputType> {
    let mut inputs = vec![];
    let input = Input::godot_singleton();
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_UP)) {
        inputs.push(InputType::Held(EngineInputs::MoveUp));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_DOWN)) {
        inputs.push(InputType::Held(EngineInputs::MoveDown));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_LEFT)) {
        inputs.push(InputType::Held(EngineInputs::MoveLeft));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_RIGHT)) {
        inputs.push(InputType::Held(EngineInputs::MoveRight));
    }

    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::HORZ_ATK)) {
        inputs.push(InputType::Held(EngineInputs::HorizontalAttack));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::VERT_ATK)) {
        inputs.push(InputType::Held(EngineInputs::VerticalAttack));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::JUMP)) {
        inputs.push(InputType::Held(EngineInputs::Jump));
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::DODGE)) {
        inputs.push(InputType::Held(EngineInputs::Dodge));
    }

    return inputs;
}
