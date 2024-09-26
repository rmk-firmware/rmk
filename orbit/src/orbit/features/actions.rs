use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::modifiers::*;

#[allow(dead_code)]
#[repr(u8)]
pub enum Actions {
  Layers,
  Mouse,
}

#[cfg(feature = "action_mouse_enabled")]
mod mouse;

#[cfg(feature = "action_layers_enabled")]
mod layers;

impl Actions {
  #[allow(dead_code)]
  #[allow(unused)]
  pub fn process(keyboard: &mut Keyboard, key: &mut Key) {
    if key.is_pressed() {
      keyboard.register_keycode(ls(4));
    } else {
      keyboard.unregister_keycode(ls(4));
    }
  }
}
