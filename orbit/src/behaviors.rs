use crate::orbit::key::Key;
use crate::orbit::keyboard::Keyboard;

#[allow(dead_code)]
#[repr(u8)]
pub enum Behavior {
  Press,
  Hold,
  Tap,
  Modding,
}

#[cfg(feature = "behavior_hold_enabled")]
mod hold;

#[cfg(feature = "behavior_tap_enabled")]
mod tap;

// normally key is sent directly after behaviors
// if its set to send_next or send_now,
// it will omit the normal press behavior
impl Behavior {
  #[allow(unused_variables)]
  pub fn process(keyboard: &mut Keyboard, key: &mut Key) {
    #[cfg(feature = "behavior_tap_enabled")]
    if key.has_behavior(Behavior::Tap) {
      tap::process(keyboard, key);
    }

    #[cfg(feature = "behavior_hold_enabled")]
    if key.has_behavior(Behavior::Hold) {
      hold::process(keyboard, key);
    }
  }
}
