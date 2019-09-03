//! Typedefs for input shortcuts.
use ggez::event::*;
use ggez_goodies::input;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
        Select,
        Menu,
        Quit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
        Vert,
        Horz,
}

#[derive(Debug)]
pub struct MouseEvent {
        pub x: f32,
        pub y: f32,
        pub dx: f32,
        pub dy: f32,
}

#[derive(Debug)]
pub enum InputEvent {
        InputEffect(input::InputEffect<Axis, Button>),
        MouseEffect(MouseEvent),
}

pub type Binding = input::InputBinding<Axis, Button>;
pub type Event = InputEvent;
pub type State = input::InputState<Axis, Button>;

/// Create the default keybindings for our input state.
pub fn create_input_binding() -> input::InputBinding<Axis, Button> {
        input::InputBinding::new()
                .bind_key_to_axis(KeyCode::Up, Axis::Vert, true)
                .bind_key_to_axis(KeyCode::Down, Axis::Vert, false)
                .bind_key_to_axis(KeyCode::Left, Axis::Horz, false)
                .bind_key_to_axis(KeyCode::Right, Axis::Horz, true)
                .bind_key_to_button(KeyCode::Z, Button::Menu)
                .bind_key_to_button(KeyCode::Escape, Button::Quit)
}
