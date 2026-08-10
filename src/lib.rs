use bevy_app::{App, MainScheduleOrder, Plugin, PreUpdate};
use bevy_ecs::schedule::{Schedule, ScheduleLabel};
use pointer_order::PointerOrderPlugin;

pub mod prelude {
    pub use super::{
        InputExt, InputExtPlugin,
        pointer_order::{OrderedPointer, PointerOrder},
    };
}

pub mod pointer_order;

pub struct InputExtPlugin;

#[derive(ScheduleLabel, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputExt;

impl Plugin for InputExtPlugin {
    fn build(&self, app: &mut App) {
        app.add_schedule(Schedule::new(InputExt));
        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_after(PreUpdate, InputExt);

        app.add_plugins(PointerOrderPlugin);
    }
}
