//! Function to spawn a "Schnecke" entity

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
/// Marker struct for the "Schnecke" entity
pub struct Schnecke;

/// spawn a "Schnecke" entity
/// also adds the `RigidBody`, `Collider`, `ActiveEvents` and `Sensor` components for collision detection
pub fn spawn_schnecke(mut commands: Commands, asset_server: Res<AssetServer>, pos: Vec3) {
    commands.spawn((
        RigidBody::Fixed,
        //Collider::cuboid(180., 110.),
        Collider::ball(90.),
        ActiveEvents::COLLISION_EVENTS,
        Sensor,
        SpriteBundle {
            sprite: Sprite {
                color: default(),
                custom_size: Some(Vec2::new(360.0, 220.0)),
                flip_x: true,
                ..default()
            },
            transform: Transform {
                translation: pos,
                rotation: Quat::from_rotation_y(180.0),
                scale: Vec3::new(0.5, 0.5, 0.5),
            },
            texture: asset_server.load("textures/snail.png"),
            ..default()
        },
        Schnecke,
    ));
}
