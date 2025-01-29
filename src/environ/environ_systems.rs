use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use crate::floorplan::FloorPlanEvent;

use super::environ_component::Ground;

// Define window size and environment constants
pub const WINDOW_WIDTH: f32 = 1200.0;
pub const WINDOW_HEIGHT: f32 = 800.0;
pub const BOUNDARY_THICKNESS: f32 = 0.1;
pub const BOUNCE_EFFECT: f32 = 0.4;

pub const WALL_DISTANCE_FROM_CENTER: f32 = 1500.0;
pub const FLOOR_CEILING_WIDTH: f32 = WALL_DISTANCE_FROM_CENTER * 2.0;

pub fn handle_floor_plan_changes(
    mut floor_plan_events: EventReader<FloorPlanEvent>,
    //mut _events: EventWriter<PlatformEvent>,
) {
    for _event in floor_plan_events.read() {
        info!("Floor plan event received!");
        // todo:
        // 1. Get the floor plan from the event
        // 2. Get the rooms and doors from the floor plan
        // 3. Calculate the size of the room based on the number of doors
        // 4. setup the environment
        // 5. calculate the placement of all the platforms
        // 6. spawn the platforms by firing the platform events with the position info
    }
}

pub fn setup_environment(mut commands: Commands) {
    // Spawn the ground with physics and centralized boundary thickness
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            FLOOR_CEILING_WIDTH / 2.0,
            BOUNDARY_THICKNESS,
        )),
        Transform::from_xyz(0.0, -WINDOW_HEIGHT / 2.0 + BOUNDARY_THICKNESS, 0.0),
        Friction {
            dynamic_coefficient: 0.8,
            static_coefficient: 0.9,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT % 2.0,
            combine_rule: CoefficientCombine::Max,
        },
        Ground,
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(-WALL_DISTANCE_FROM_CENTER, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(BOUNDARY_THICKNESS * 200.0, WINDOW_HEIGHT)),
            ..default()
        },
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(BOUNDARY_THICKNESS, WINDOW_HEIGHT / 2.0)),
        Transform::from_xyz(WALL_DISTANCE_FROM_CENTER, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(BOUNDARY_THICKNESS * 200.0, WINDOW_HEIGHT)),
            ..default()
        },
    ));
    // Top boundary
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            FLOOR_CEILING_WIDTH / 2.0,
            BOUNDARY_THICKNESS,
        )),
        Transform::from_xyz(0.0, WINDOW_HEIGHT / 2.0 - BOUNDARY_THICKNESS, 0.0),
        Restitution {
            coefficient: BOUNCE_EFFECT,
            combine_rule: CoefficientCombine::Max,
        },
    ));
}
