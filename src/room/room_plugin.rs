use bevy::prelude::*;

use crate::scheduler::InGameSet;

use super::{
    room_component::{CurrentFloorPlan, RoomState, WINDOW_HEIGHT, WINDOW_WIDTH},
    room_systems::{handle_floor_plan_changes, setup_room, update_doors, update_room},
};
/// define the game window size and environment constants and create the left and right walls, the
/// ground, and the top boundary.
pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .insert_resource(RoomState::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Doors".to_string(),
                    resolution: bevy::window::WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                    ..default()
                }),
                ..default()
            }))
            //.add_systems(Startup, setup_room)
            .add_systems(
                Update,
                (
                    setup_room,
                    handle_floor_plan_changes,
                    update_doors,
                    update_room,
                )
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}
