mod controls;
mod texts;

use bevy_mod_picking::events::{Drag, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};

use controls::{Mausi, Schnecke};

use texts::MyTextPlugin;

use bevy::prelude::*;

use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(MyTextPlugin)
        .add_systems(Startup, setup_schnecke_and_mausi)
 
        .add_systems(
            Update,
            (
                move_schnecke,
                listen_for_collition_events,
     
            ),
        )
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .insert_resource(RapierConfiguration {
        //     gravity: Vec2::ZERO,
        //     ..default()
        // })
        .run();
}

fn setup_schnecke_and_mausi(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle { ..default() });

    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(180., 110.),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.9, 0.1),
                    custom_size: Some(Vec2::new(360.0, 220.0)),
                    ..default()
                },
                transform: Transform::from_xyz(400.0, 0.0, 0.0),

                texture: asset_server.load("textures/maus.png"),
                ..default()
            },
            Mausi,
            //make Mausi dgraggable:
            PickableBundle::default(),
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x;
                transform.translation.y -= drag.delta.y;
            }),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor);

    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::cuboid(180., 110.),
            SpriteBundle {
                sprite: Sprite {
                    color: default(),
                    custom_size: Some(Vec2::new(360.0, 220.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(-700.0, -300.0, 0.0),
                    rotation: Quat::from_rotation_y(180.0),
                    scale: Vec3::new(0.5, 0.5, 0.5),
                },

                texture: asset_server.load("textures/snail.png"),
                ..default()
            },
            Schnecke,
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor);
}

//SYSTEMS:

fn move_schnecke(mut query: Query<&mut Transform, With<Schnecke>>, time: Res<Time>) {
    let speed = 45.0;
    let window_width = 650.0;

    for mut transform in query.iter_mut() {
        let movement = Vec3::new(speed * time.delta().as_secs_f32(), 0.0, 0.0);

        let current_position = transform.translation.x;
        if current_position + movement.x > window_width {
            transform.translation = Vec3::new(-700.0, -300.0, 0.0);
        }

        transform.translation += movement;
    }
}

fn listen_for_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<Entity, With<Schnecke>>,
) {
    // Create a vector to store entities involved in collisions
    let mut collided_entities: Vec<Entity> = Vec::new();

    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Add the entities involved in the collision to the vector
                collided_entities.push(entity1);
                collided_entities.push(entity2);

                println!("CollisionEvent: Started");

                for schnecke in query.iter() {
                    if collided_entities.contains(&schnecke) {
                        commands.entity(schnecke).despawn_recursive();
                    }
                }

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/beep.mp3"),
                    settings: PlaybackSettings::ONCE,
                });
            }
            CollisionEvent::Stopped(_, _, _) => println!("CollisionEvent: Stopped"),
        }
    }

}
