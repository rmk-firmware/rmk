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
      keyboard.add_report(ls(5));
    } else {
      keyboard.remove_report(ls(5));
    }
  }
}
