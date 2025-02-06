use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;

use super::room_component::{
    Ceiling, CurrentFloorPlan, Floor, LeftWall, RightWall, RoomState, WINDOW_HEIGHT,
};
use crate::{
    floorplan::{Door, FloorPlan, FloorPlanEvent, Room},
    room::room_component::DoorState,
    state::GameState,
};

const PLATFORM_X_SEPARATOR: f32 = 450.0;
const PLATFORM_Y_SEPARATOR: &[f32] = &[
    0.0, -150.0, 250.0, -100.0, -210.0, 100.0, -200.0, -50.0, 175.0, 25.0,
];

pub fn handle_floor_plan_changes(
    mut floorplan_events: EventReader<FloorPlanEvent>,
    mut current_floorplan: ResMut<CurrentFloorPlan>,
) {
    for event in floorplan_events.read() {
        debug!("Floor plan event received.");

        let new_floorplan = event.floorplan.clone();

        if let Some(current_plan) = &current_floorplan.floorplan {
            if *current_plan != new_floorplan {
                warn!("Floor plan has changed.");
                // Calculate the differences and fire other events
                // For example, you can spawn new platforms based on the differences
                // commands.spawn(...);
            }
        }

        let (you_are_here, you_were_here) =
            determine_current_location(&new_floorplan, &current_floorplan);

        // Update the current floor plan
        *current_floorplan = CurrentFloorPlan {
            floorplan: Some(new_floorplan),
            you_are_here,
            you_were_here,
        };
    }
}

/// if there is no current room location, then it is the start room of the new floor plan
fn determine_current_location(
    new_floorplan: &FloorPlan,
    current_floorplan: &CurrentFloorPlan,
) -> (Option<String>, Option<String>) {
    current_floorplan.you_are_here.as_ref().map_or_else(
        || {
            new_floorplan.get_start_room().map_or_else(
                |_| {
                    warn!("No start room found in the floor plan.");
                    (None, current_floorplan.you_were_here.clone())
                },
                |room| {
                    (
                        Some(room.id.clone()),
                        current_floorplan.you_are_here.clone(),
                    )
                },
            )
        },
        |location| {
            (
                Some(location.clone()),
                current_floorplan.you_were_here.clone(),
            )
        },
    )
}

pub fn update_doors(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,

    current_floorplan: Res<CurrentFloorPlan>,
    mut room_state: ResMut<RoomState>,
) {
    if !current_floorplan.is_changed() {
        return;
    }
    room_state.doors.clear();
    room_state
        .previous_room_id
        .clone_from(&current_floorplan.you_were_here);

    if let Some(floorplan) = current_floorplan.floorplan.as_ref() {
        if let Some(room_id) = &current_floorplan.you_are_here {
            match floorplan.get_doors_and_connected_rooms(room_id) {
                Ok(doors_and_rooms) => {
                    update_room_state_with_doors(&mut room_state, doors_and_rooms);
                }
                _ => panic!("Failed to get doors and connected rooms"),
            }
        }
    }

    if *state == GameState::RoomChange {
        next_state.set(GameState::TransitioningIn);
    }
}

#[allow(clippy::cast_precision_loss)]
fn update_room_state_with_doors(room_state: &mut RoomState, doors_and_rooms: Vec<(&Door, &Room)>) {
    let number_of_doors = doors_and_rooms.len();
    let room_width = PLATFORM_X_SEPARATOR * (number_of_doors + 1) as f32;
    room_state.wall_distance_from_center = room_width / 2.0;
    room_state.floor_ceiling_width = room_width;

    let mut room_seq = 0;
    let mut room_positions: Vec<Vec2> = (0..number_of_doors)
        .map(|i| {
            let y_index = if room_seq < PLATFORM_Y_SEPARATOR.len() {
                room_seq
            } else {
                room_seq % PLATFORM_Y_SEPARATOR.len()
            };
            room_seq += 1;

            Vec2 {
                x: PLATFORM_X_SEPARATOR.mul_add(i as f32, PLATFORM_X_SEPARATOR)
                    - room_state.wall_distance_from_center,
                y: PLATFORM_Y_SEPARATOR[y_index],
            }
        })
        .collect();

    for (_, room) in doors_and_rooms {
        if let Some(position) = room_positions.pop() {
            let door_state = DoorState {
                room_id: room.id.clone(),
                room_name: room.name.clone(),
                position,
            };
            room_state.doors.push(door_state);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_room(
    room_state: Res<RoomState>,
    mut param_set: ParamSet<(
        Query<(&mut Transform, &mut Collider), With<Floor>>,
        Query<(&mut Transform, &mut Collider, &mut Sprite), With<LeftWall>>,
        Query<(&mut Transform, &mut Collider, &mut Sprite), With<RightWall>>,
        Query<(&mut Transform, &mut Collider), With<Ceiling>>,
    )>,
) {
    if !room_state.is_changed() {
        return;
    }
    debug!("Updating room...");

    update_ground(&room_state, &mut param_set.p0());
    update_left_wall(
        &room_state,
        &mut param_set.p1(),
        -room_state.wall_distance_from_center,
    );
    update_right_wall(
        &room_state,
        &mut param_set.p2(),
        room_state.wall_distance_from_center,
    );
    update_ceiling(&room_state, &mut param_set.p3());
}

fn update_ground(
    room_state: &RoomState,
    query: &mut Query<(&mut Transform, &mut Collider), With<Floor>>,
) {
    for (mut transform, mut collider) in query.iter_mut() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width / 2.0,
            room_state.boundary_thickness,
        ));
        *transform = Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + room_state.boundary_thickness,
            0.0,
        );
    }
}

fn update_left_wall(
    room_state: &RoomState,
    query: &mut Query<(&mut Transform, &mut Collider, &mut Sprite), With<LeftWall>>,
    x_position: f32,
) {
    for (mut transform, mut collider, mut sprite) in query.iter_mut() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        ));
        *transform = Transform::from_xyz(x_position, 0.0, 0.0);
        sprite.custom_size = Some(Vec2::new(
            room_state.boundary_thickness * 200.0,
            WINDOW_HEIGHT,
        ));
    }
}

fn update_right_wall(
    room_state: &RoomState,
    query: &mut Query<(&mut Transform, &mut Collider, &mut Sprite), With<RightWall>>,
    x_position: f32,
) {
    for (mut transform, mut collider, mut sprite) in query.iter_mut() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        ));
        *transform = Transform::from_xyz(x_position, 0.0, 0.0);
        sprite.custom_size = Some(Vec2::new(
            room_state.boundary_thickness * 200.0,
            WINDOW_HEIGHT,
        ));
    }
}

fn update_ceiling(
    room_state: &RoomState,
    query: &mut Query<(&mut Transform, &mut Collider), With<Ceiling>>,
) {
    for (mut transform, mut collider) in query.iter_mut() {
        *collider = Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width / 2.0,
            room_state.boundary_thickness,
        ));
        *transform = Transform::from_xyz(
            0.0,
            WINDOW_HEIGHT / 2.0 - room_state.boundary_thickness,
            0.0,
        );
    }
}

pub fn setup_room(
    mut commands: Commands,
    room_state: ResMut<RoomState>,
    current_floorplan: Res<CurrentFloorPlan>,
) {
    if current_floorplan.you_are_here.is_some() {
        // room is already setup
        return;
    }

    spawn_floor(&mut commands, &room_state);
    spawn_wall(
        &mut commands,
        &room_state,
        -room_state.wall_distance_from_center,
        LeftWall,
    );
    spawn_wall(
        &mut commands,
        &room_state,
        room_state.wall_distance_from_center,
        RightWall,
    );
    spawn_ceiling(&mut commands, &room_state);
}

fn spawn_floor(commands: &mut Commands, room_state: &RoomState) {
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width,
            room_state.boundary_thickness,
        )),
        Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + room_state.boundary_thickness,
            0.0,
        ),
        Friction {
            dynamic_coefficient: 0.8,
            static_coefficient: 0.9,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: room_state.bounce_effect % 2.0,
            combine_rule: CoefficientCombine::Max,
        },
        Floor,
    ));
}

fn spawn_wall(
    commands: &mut Commands,
    room_state: &RoomState,
    x_position: f32,
    wall_type: impl Component,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            room_state.boundary_thickness,
            WINDOW_HEIGHT / 2.0,
        )),
        Transform::from_xyz(x_position, 0.0, 0.0),
        Friction {
            dynamic_coefficient: 0.5,
            static_coefficient: 0.6,
            combine_rule: CoefficientCombine::Average,
        },
        Restitution {
            coefficient: room_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5), // Matching the platform color
            custom_size: Some(Vec2::new(
                room_state.boundary_thickness * 200.0,
                WINDOW_HEIGHT,
            )),
            ..default()
        },
        wall_type,
    ));
}

fn spawn_ceiling(commands: &mut Commands, room_state: &RoomState) {
    commands.spawn((
        RigidBody::Static,
        Collider::from(SharedShape::cuboid(
            room_state.floor_ceiling_width,
            room_state.boundary_thickness,
        )),
        Transform::from_xyz(
            0.0,
            WINDOW_HEIGHT / 2.0 - room_state.boundary_thickness,
            0.0,
        ),
        Restitution {
            coefficient: room_state.bounce_effect,
            combine_rule: CoefficientCombine::Max,
        },
        Ceiling,
    ));
}
