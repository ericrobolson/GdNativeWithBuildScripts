mod character;

use fixed;
use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameEngine {
    frame: usize,
}

enum EngineInputs {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    HeavyAttack,
    LightAttack,
    Block,
    Dodge,
}

#[methods]
impl GameEngine {
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        Self { frame: 0 }
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
    fn _ready(&self, _owner: Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("hello, world. This is an engine test.");
    }

    #[export]
    unsafe fn _physics_process(&mut self, mut owner: Node, delta: f64) {
        let input = get_mapped_input();

        if input.is_empty() == false {
            self.frame += 1;
            godot_print!("Tick: {}", self.frame);
            //Do execution stuff
        }
    }
}

fn get_mapped_input() -> Vec<EngineInputs> {
    // TODO: change this from a 'polling' pattern to a 'event' pattern to collect all inputs that occured.
    // Then, push them into the engine? Not sure if that's the direction I want to take or not. For now, don't worry about it.
    let input = Input::godot_singleton();

    let mut inputs = vec![];

    if Input::is_action_pressed(&input, GodotString::from_str("character_move_up")) {
        inputs.push(EngineInputs::MoveUp);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_move_down")) {
        inputs.push(EngineInputs::MoveDown);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_move_left")) {
        inputs.push(EngineInputs::MoveLeft);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_move_right")) {
        inputs.push(EngineInputs::MoveRight);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_dodge")) {
        inputs.push(EngineInputs::Dodge);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_heavy_attack")) {
        inputs.push(EngineInputs::HeavyAttack);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_light_attack")) {
        inputs.push(EngineInputs::LightAttack);
    }
    if Input::is_action_pressed(&input, GodotString::from_str("character_block")) {
        inputs.push(EngineInputs::Block);
    }

    return inputs;
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameEngine>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
