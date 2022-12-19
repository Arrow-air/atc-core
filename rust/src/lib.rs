//! ATC-CORE implemented in Rust for Godot

use gdnative::prelude::*;
mod common;
mod assets;
mod world;

/// Holds the running instance of atc-core
#[derive(NativeClass)]
#[inherit(Node)]
#[derive(Copy, Clone, Debug)]
pub struct AtcCore;

#[methods]
impl AtcCore {
    fn new(_owner: &Node) -> Self {
        AtcCore
    }

    #[method]
    fn _ready(&self, #[base] _owner: &Node) {
        godot_print!("hello, world.");
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<AtcCore>();
    handle.add_class::<world::World>();
}

godot_init!(init);
