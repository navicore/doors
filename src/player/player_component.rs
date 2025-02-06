use avian2d::{parry::shape::SharedShape, prelude::*};
use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

// Define movement constants
pub const PLAYER_MOVE_SPEED: f32 = 500.0; // Horizontal movement speed
pub const PLAYER_JUMP_FORCE: f32 = 25000.0; // Jump force applied when pressing space
pub const PLAYER_GRAVITY_SCALE: f32 = 25.0; // Gravity multiplier for falling speed

static PLAYER_COLOR: Color = Color::srgb(0.3, 0.6, 1.0); // light blue

#[derive(Bundle)]
pub struct PlayerBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub external_force: ExternalForce,
    pub gravity: GravityScale,
    pub mass: Mass,
    pub friction: Friction,
    pub sprite: Sprite,
    pub player: Player,
    pub movable: Movable,
    pub grounded: Grounded,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::from(SharedShape::cuboid(40.0, 40.0)),
            external_force: ExternalForce::default(),
            gravity: GravityScale(PLAYER_GRAVITY_SCALE),
            mass: Mass(1.0),
            friction: Friction {
                dynamic_coefficient: 0.3,
                static_coefficient: 0.5,
                combine_rule: CoefficientCombine::Average,
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            player: Player,
            movable: Movable,
            grounded: Grounded(false),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Action {
    MoveLeft,
    MoveRight,
    Jump,
    Enter,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component)]
pub struct Movable;
