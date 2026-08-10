use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;
use pointer_order::PointerOrderPlugin;

pub mod prelude {
    pub use super::{
        InputExtPlugin, InputExtSystems,
        pointer_order::{OrderedPointer, PointerOrder},
    };
}

pub mod pointer_order;

pub struct InputExtPlugin;

#[derive(SystemSet, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputExtSystems;

impl Plugin for InputExtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PointerOrderPlugin);
    }
}
