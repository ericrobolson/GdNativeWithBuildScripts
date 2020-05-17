pub mod lib_core;
use lib_core::{EngineInputs, InputType};
mod ecs;
use fixed;
use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameEngine {
    frame: usize,
    executed_frames: usize,
    player_inputs: Vec<EngineInputs>,
    world: ecs::World,
}

#[methods]
impl GameEngine {
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        Self {
            frame: 0,
            executed_frames: 0,
            player_inputs: vec![],
            world: ecs::World::new(),
        }
    }

    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: Node) {}

    #[export]
    unsafe fn _input(&mut self, mut owner: Node, event: InputEvent) {
        const MOVE_UP: &str = "character_move_up";
        const MOVE_DOWN: &str = "character_move_down";
        const MOVE_LEFT: &str = "character_move_left";
        const MOVE_RIGHT: &str = "character_move_right";

        const DODGE: &str = "character_dodge";
        const BLOCK: &str = "character_block";

        const HEAVY_ATK: &str = "character_heavy_attack";
        const LIGHT_ATK: &str = "character_light_attack";

        // Move Up
        if event.is_action_pressed(GodotString::from_str(MOVE_UP), false) {
            self.player_inputs
                .push(EngineInputs::MoveUp(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(MOVE_UP)) {
            self.player_inputs
                .push(EngineInputs::MoveUp(InputType::Released));
        }
        // Move Down
        else if event.is_action_pressed(GodotString::from_str(MOVE_DOWN), false) {
            self.player_inputs
                .push(EngineInputs::MoveDown(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(MOVE_DOWN)) {
            self.player_inputs
                .push(EngineInputs::MoveDown(InputType::Released));
        }
        // Move Left
        else if event.is_action_pressed(GodotString::from_str(MOVE_LEFT), false) {
            self.player_inputs
                .push(EngineInputs::MoveLeft(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(MOVE_LEFT)) {
            self.player_inputs
                .push(EngineInputs::MoveLeft(InputType::Released));
        }
        // Move Right
        else if event.is_action_pressed(GodotString::from_str(MOVE_RIGHT), false) {
            self.player_inputs
                .push(EngineInputs::MoveRight(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(MOVE_RIGHT)) {
            self.player_inputs
                .push(EngineInputs::MoveRight(InputType::Released));
        }
        // Dodge
        else if event.is_action_pressed(GodotString::from_str(DODGE), false) {
            self.player_inputs
                .push(EngineInputs::Dodge(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(DODGE)) {
            self.player_inputs
                .push(EngineInputs::Dodge(InputType::Released));
        }
        // Block
        else if event.is_action_pressed(GodotString::from_str(BLOCK), false) {
            self.player_inputs
                .push(EngineInputs::Block(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(BLOCK)) {
            self.player_inputs
                .push(EngineInputs::Block(InputType::Released));
        }
        // Heavy Atk
        else if event.is_action_pressed(GodotString::from_str(HEAVY_ATK), false) {
            self.player_inputs
                .push(EngineInputs::HeavyAttack(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(HEAVY_ATK)) {
            self.player_inputs
                .push(EngineInputs::HeavyAttack(InputType::Released));
        }
        // Light Atk
        else if event.is_action_pressed(GodotString::from_str(LIGHT_ATK), false) {
            self.player_inputs
                .push(EngineInputs::LightAttack(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(LIGHT_ATK)) {
            self.player_inputs
                .push(EngineInputs::LightAttack(InputType::Released));
        }
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: Node, delta: f64) {
        self.frame += 1;

        if self.player_inputs.is_empty() == false {
            self.executed_frames += 1;
            godot_print!("f: {}", self.executed_frames);

            //TODO: stuff with engine inputs
            self.world.dispatch();

            self.player_inputs.clear();
        }
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameEngine>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
