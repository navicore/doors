use bevy::prelude::*;

use super::pause_system::{
    display_paused_text, handle_pause_events, pause_game, remove_pause_text,
};
use crate::state::GameState::Paused;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (pause_game, handle_pause_events))
            .add_systems(OnEnter(Paused), display_paused_text)
            .add_systems(OnExit(Paused), remove_pause_text);
    }
}
