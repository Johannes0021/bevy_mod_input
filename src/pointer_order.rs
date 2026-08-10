use super::InputExtSystems;
use bevy_app::{App, Plugin, PreUpdate};
use bevy_asset::uuid::Uuid;
use bevy_ecs::{
    change_detection::ResMut, message::MessageReader, resource::Resource,
    schedule::IntoScheduleConfigs,
};
use bevy_input::{
    ButtonState,
    mouse::MouseButton,
    mouse::MouseButtonInput,
    touch::{TouchInput, TouchPhase},
};
use bevy_picking::pointer::PointerId;

pub struct PointerOrderPlugin;

impl Plugin for PointerOrderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PointerOrder>()
            .add_systems(PreUpdate, track_pointer_order.in_set(InputExtSystems));
    }
}

fn track_pointer_order(
    mut pointer_order: ResMut<PointerOrder>,
    mut mouse_events: MessageReader<MouseButtonInput>,
    mut touch_events: MessageReader<TouchInput>,
) {
    // Mouse
    for event in mouse_events.read() {
        match event.state {
            ButtonState::Pressed => {
                pointer_order.add(event.button.into());
            }
            ButtonState::Released => {
                pointer_order.remove(event.button.into());
            }
        }
    }

    // Touch
    for event in touch_events.read() {
        let pointer = OrderedPointer::Touch(event.id);

        match event.phase {
            TouchPhase::Started => {
                pointer_order.add(pointer);
            }
            TouchPhase::Ended | TouchPhase::Canceled => {
                pointer_order.remove(pointer);
            }
            TouchPhase::Moved => (),
        }
    }
}

//==================================================================================================
// OrderedPointer
//==================================================================================================

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum OrderedPointer {
    Mouse(MouseButton),
    Touch(u64),
    Custom(Uuid),
}

impl From<MouseButton> for OrderedPointer {
    fn from(button: MouseButton) -> Self {
        Self::Mouse(button)
    }
}

impl From<OrderedPointer> for PointerId {
    fn from(pointer: OrderedPointer) -> Self {
        match pointer {
            OrderedPointer::Mouse(_) => Self::Mouse,
            OrderedPointer::Touch(id) => Self::Touch(id),
            OrderedPointer::Custom(id) => Self::Custom(id),
        }
    }
}

//==================================================================================================
// PointerOrder
//==================================================================================================

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash)]
pub struct PointerOrder {
    pointers: Vec<Option<OrderedPointer>>,
}

impl Default for PointerOrder {
    fn default() -> Self {
        Self {
            pointers: Vec::with_capacity(16),
        }
    }
}

impl PointerOrder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, index: usize) -> Option<OrderedPointer> {
        self.pointers.get(index).copied().flatten()
    }

    pub fn index_of(&self, pointer: OrderedPointer) -> Option<usize> {
        self.pointers
            .iter()
            .position(|current| *current == Some(pointer))
    }

    pub fn indices_of_pointer_id(&self, pointer: PointerId) -> Vec<usize> {
        match pointer {
            PointerId::Mouse => self
                .pointers
                .iter()
                .enumerate()
                .filter_map(|(index, pointer)| {
                    matches!(pointer, Some(OrderedPointer::Mouse(_))).then_some(index)
                })
                .collect(),

            PointerId::Touch(id) => self
                .index_of(OrderedPointer::Touch(id))
                .map_or_else(Vec::new, |i| vec![i]),

            PointerId::Custom(id) => self
                .index_of(OrderedPointer::Custom(id))
                .map_or_else(Vec::new, |i| vec![i]),
        }
    }

    pub fn index_of_mouse(&self, button: MouseButton) -> Option<usize> {
        self.index_of(OrderedPointer::Mouse(button))
    }

    pub fn is_at(&self, pointer: OrderedPointer, index: usize) -> bool {
        self.get(index) == Some(pointer)
    }

    pub fn is_pointer_id_at(&self, pointer: PointerId, index: usize) -> bool {
        match pointer {
            PointerId::Mouse => {
                matches!(self.get(index), Some(OrderedPointer::Mouse(_)))
            }
            PointerId::Touch(id) => self.get(index) == Some(OrderedPointer::Touch(id)),
            PointerId::Custom(id) => self.get(index) == Some(OrderedPointer::Custom(id)),
        }
    }

    pub fn is_mouse_at(&self, button: MouseButton, index: usize) -> bool {
        self.get(index) == Some(OrderedPointer::Mouse(button))
    }

    pub fn pointers(&self) -> &[Option<OrderedPointer>] {
        &self.pointers
    }

    pub fn add(&mut self, pointer: OrderedPointer) -> usize {
        if let Some(index) = self.index_of(pointer) {
            return index;
        }

        if let Some(index) = self.pointers.iter().position(Option::is_none) {
            self.pointers[index] = Some(pointer);
            return index;
        }

        let index = self.pointers.len();
        self.pointers.push(Some(pointer));

        index
    }

    pub fn remove(&mut self, pointer: OrderedPointer) -> Option<usize> {
        let index = self.index_of(pointer)?;

        self.pointers[index] = None;

        Some(index)
    }

    pub fn contains(&self, pointer: OrderedPointer) -> bool {
        self.index_of(pointer).is_some()
    }

    pub fn contains_pointer_id(&self, pointer: PointerId) -> bool {
        !self.indices_of_pointer_id(pointer).is_empty()
    }

    pub fn contains_mouse(&self, button: MouseButton) -> bool {
        self.index_of_mouse(button).is_some()
    }
}
