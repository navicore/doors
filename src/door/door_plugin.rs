use bevy::prelude::*;

use crate::{scheduler::InGameSet, state::GameState};

use super::door_systems::spawn_platforms;

/// create platforms for the player to jump on.  platforms tend to have doors on top of them.
pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_platforms
                .run_if(in_state(GameState::InGame))
                .in_set(InGameSet::EntityUpdates),
        );
    }
}
