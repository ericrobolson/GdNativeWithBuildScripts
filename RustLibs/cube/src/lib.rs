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
    player_inputs: Vec<InputType>,
    world: ecs::World,
}

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
}

fn get_input_from_event(
    button: &'static str,
    input: EngineInputs,
    event: InputEvent,
) -> Option<InputType> {
    if event.is_action_pressed(GodotString::from_str(button), false) {
        return Some(InputType::Pressed(input));
    } else if event.is_action_released(GodotString::from_str(button)) {
        return Some(InputType::Released(input));
    }

    return None;
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
        let up = get_input_from_event(InputConst::MOVE_UP, EngineInputs::MoveUp, event);
        if up.is_some() {
            self.player_inputs.push(up.unwrap());
            return;
        }
        // Move Down
        let down = get_input_from_event(InputConst::MOVE_DOWN, EngineInputs::MoveDown, event);
        if down.is_some() {
            self.player_inputs.push(down.unwrap());
            return;
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
            //TODO: what happens when an entity is deleted?
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

    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::HORZ_LITE_ATK)) {
        inputs.push(EngineInputs::HorizontalLightAttack(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::HORZ_HEAVY_ATK)) {
        inputs.push(EngineInputs::HorizontalHeavyAttack(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::VERT_LITE_ATK)) {
        inputs.push(EngineInputs::VerticalLightAttack(InputType::Held))
    }
    if Input::is_action_pressed(&input, GodotString::from_str(InputConst::VERT_HEAVY_ATK)) {
        inputs.push(EngineInputs::VerticalHeavyAttack(InputType::Held))
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
