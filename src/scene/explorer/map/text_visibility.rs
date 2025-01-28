use bevy::prelude::*;

use crate::scene::explorer::event::TextVisibilityEvent;

pub fn text_visibility_reader(mut event: EventReader<TextVisibilityEvent>) {
    for e in event.read() {
        match e {
            TextVisibilityEvent::KeyPressToggle => todo!(),
            TextVisibilityEvent::ButtonToggle => todo!(),
            TextVisibilityEvent::Zoom => todo!(),
        }
    }
}
