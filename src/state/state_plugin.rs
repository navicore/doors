use bevy::prelude::*;

use super::{
    state_component::GameState,
    state_system::{fade_in, fade_out, setup_fade_overlay},
};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Startup, setup_fade_overlay)
            .add_systems(
                Update,
                fade_out.run_if(in_state(GameState::TransitioningOut)),
            )
            .add_systems(Update, fade_in.run_if(in_state(GameState::TransitioningIn)));
    }
}
