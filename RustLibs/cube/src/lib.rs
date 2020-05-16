use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GameEngine;

#[methods]
impl GameEngine {
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        Self
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
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<GameEngine>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
