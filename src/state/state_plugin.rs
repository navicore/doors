use super::{
    state_component::GameState,
    state_component::GameState::{RoomChange, TransitioningIn, TransitioningOut},
    state_system::{
        fade_in, fade_out, room_change_curtain, setup_fade_overlay, update_fade_overlay,
    },
};
use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup_fade_overlay)
            .add_systems(Update, update_fade_overlay)
            .add_systems(Update, fade_out.run_if(in_state(TransitioningOut)))
            .add_systems(Update, room_change_curtain.run_if(in_state(RoomChange)))
            .add_systems(Update, fade_in.run_if(in_state(TransitioningIn)));
    }
}
