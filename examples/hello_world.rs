use bevy::prelude::*;
use bevy_mod_input::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputExtPlugin)
        .add_systems(Update, print_pointer_order)
        .run();
}

fn print_pointer_order(pointer_order: Res<PointerOrder>) {
    if pointer_order.is_changed() {
        info!(
            "Pointer order: [{}]",
            pointer_order
                .pointers()
                .iter()
                .map(|pointer| format!("{pointer:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
