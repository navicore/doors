use crate::scheduler::InGameSet;
use crate::state::GameState;
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use super::{
    player_component::Action,
    player_systems::{detect_player_at_door, player_movement, spawn_player},
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(
                Update,
                detect_player_at_door
                    .in_set(InGameSet::UserInput)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                player_movement
                    .in_set(InGameSet::UserInput)
                    .run_if(in_state(GameState::InGame)),
            )
            .add_plugins(InputManagerPlugin::<Action>::default());
    }
}
