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

struct InputConst {}
impl InputConst {
    pub const MOVE_UP: &'static str = "character_move_up";
    pub const MOVE_DOWN: &'static str = "character_move_down";
    pub const MOVE_LEFT: &'static str = "character_move_left";
    pub const MOVE_RIGHT: &'static str = "character_move_right";

    pub const JUMP: &'static str = "character_jump";

    pub const HEAVY_ATK: &'static str = "character_heavy_attack";
    pub const MEDIUM_ATK: &'static str = "character_medium_attack";
    pub const LIGHT_ATK: &'static str = "character_light_attack";
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
        // Move Up
        if event.is_action_pressed(GodotString::from_str(InputConst::MOVE_UP), false) {
            self.player_inputs
                .push(EngineInputs::MoveUp(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::MOVE_UP)) {
            self.player_inputs
                .push(EngineInputs::MoveUp(InputType::Released));
        }
        // Move Down
        else if event.is_action_pressed(GodotString::from_str(InputConst::MOVE_DOWN), false) {
            self.player_inputs
                .push(EngineInputs::MoveDown(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::MOVE_DOWN)) {
            self.player_inputs
                .push(EngineInputs::MoveDown(InputType::Released));
        }
        // Move Left
        else if event.is_action_pressed(GodotString::from_str(InputConst::MOVE_LEFT), false) {
            self.player_inputs
                .push(EngineInputs::MoveLeft(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::MOVE_LEFT)) {
            self.player_inputs
                .push(EngineInputs::MoveLeft(InputType::Released));
        }
        // Move Right
        else if event.is_action_pressed(GodotString::from_str(InputConst::MOVE_RIGHT), false) {
            self.player_inputs
                .push(EngineInputs::MoveRight(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::MOVE_RIGHT)) {
            self.player_inputs
                .push(EngineInputs::MoveRight(InputType::Released));
        }
        // Jump
        else if event.is_action_pressed(GodotString::from_str(InputConst::JUMP), false) {
            self.player_inputs
                .push(EngineInputs::Jump(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::JUMP)) {
            self.player_inputs
                .push(EngineInputs::Jump(InputType::Released));
        }
        // Heavy Atk
        else if event.is_action_pressed(GodotString::from_str(InputConst::HEAVY_ATK), false) {
            self.player_inputs
                .push(EngineInputs::HeavyAttack(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::HEAVY_ATK)) {
            self.player_inputs
                .push(EngineInputs::HeavyAttack(InputType::Released));
        }
        // Heavy Atk
        else if event.is_action_pressed(GodotString::from_str(InputConst::MEDIUM_ATK), false) {
            self.player_inputs
                .push(EngineInputs::MediumAttack(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::MEDIUM_ATK)) {
            self.player_inputs
                .push(EngineInputs::MediumAttack(InputType::Released));
        }
        // Light Atk
        else if event.is_action_pressed(GodotString::from_str(InputConst::LIGHT_ATK), false) {
            self.player_inputs
                .push(EngineInputs::LightAttack(InputType::Pressed));
        } else if event.is_action_released(GodotString::from_str(InputConst::LIGHT_ATK)) {
            self.player_inputs
                .push(EngineInputs::LightAttack(InputType::Released));
        }
    }
    #[export]
    unsafe fn _physics_process(&mut self, mut owner: Node, delta: f64) {
        self.frame += 1;

        self.player_inputs.append(&mut input_poll());

        if self.player_inputs.is_empty() == false {
            self.executed_frames += 1;
            godot_print!("f: {}", self.executed_frames);

            self.world.register_player_inputs(&self.player_inputs);
            self.world.dispatch();

            self.player_inputs.clear();
        }

        self.link_to_gd_nodes(owner);
    }

    fn link_to_gd_nodes(&mut self, mut owner: Node) {
        // Create any nodes that need be created
        for e in self.world.entities() {
            let gdnode = &self.world.gd_nodes[e];
            let transform = &self.world.transforms[e];

            // Todo: what should constitute a gdnode creation?
            if transform.is_none() {
                continue;
            }

            //TODO: deletion of nodes

            // Create gdnode if it doesn't exist
            if gdnode.is_none() {
                let mut gd_node = gdnative::Node2D::new();

                unsafe {
                    gd_node.add_child(Some(gdnative::Button::new().cast().unwrap()), false);
                }

                unsafe {
                    owner.add_child(Some(unsafe { gd_node.cast().unwrap() }), false);
                };

                let id = unsafe { gd_node.get_instance_id() };
                self.world.gd_nodes[e] = Some(ecs::components::GdNodeComponent::new(id));
            }
        }

        // Get the nodes
        let children = unsafe { owner.get_children() };
        let mut children: Vec<Node> = children
            .iter()
            .filter(|c| c.has_method(&GodotString::from_str("get_instance_id")))
            .map(|c| c.try_to_object::<Node>())
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .collect();

        // Update transforms
        for e in self.world.entities() {
            let gd_node_component = &self.world.gd_nodes[e];
            if gd_node_component.is_none() {
                continue;
            }

            let transform = &self.world.transforms[e];
            if transform.is_none() {
                continue;
            }

            let transform = transform.as_ref().unwrap();

            let gd_node_component = gd_node_component.as_ref().unwrap();

            let mut gd_node = children
                .iter_mut()
                .find(|c| unsafe { c.get_instance_id() } == gd_node_component.id);

            if gd_node.is_none() {
                continue;
            }

            let gd_node = gd_node.unwrap();
            let gd_node = unsafe { gd_node.cast::<Node2D>() };
            if gd_node.is_some() {
                // Update position
                unsafe { gd_node.unwrap().set_position(transform.position.into()) };
            }
        }
    }
}

fn input_poll() -> Vec<EngineInputs> {
    let mut inputs = vec![];
    let input = Input::godot_singleton();
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_UP)) {
        inputs.push(EngineInputs::MoveUp(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_DOWN)) {
        inputs.push(EngineInputs::MoveDown(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_LEFT)) {
        inputs.push(EngineInputs::MoveLeft(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MOVE_RIGHT)) {
        inputs.push(EngineInputs::MoveRight(InputType::Held))
    }

    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::JUMP)) {
        inputs.push(EngineInputs::Jump(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::HEAVY_ATK)) {
        inputs.push(EngineInputs::HeavyAttack(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::MEDIUM_ATK)) {
        inputs.push(EngineInputs::MediumAttack(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::LIGHT_ATK)) {
        inputs.push(EngineInputs::LightAttack(InputType::Held))
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
