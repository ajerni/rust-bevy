mod controls;
mod fly_plugin;
mod schnecke;
mod scoreboard;
mod texts;
mod timers;

use bevy_mod_picking::events::{Click, Drag, Move, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};

use crate::schnecke::*;
use crate::scoreboard::*;
use controls::{AnimationStateResource, ClickDetectorPlugin, Cubie, Mausi, Spaceship};
use fly_plugin::FlyPlugin;
use texts::MyTextPlugin;
use timers::MyTimer;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct Rotator {
    speed: f32,
}

// impl Default for Rotator {
//     fn default() -> Self {
//         Self { speed: 100.0 }
//     }
// }

fn main() {
    App::new()
        .insert_resource(AnimationStateResource { moving: false })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 1.0)))
        .insert_resource(Scoreboard { score: 0 })
        .init_resource::<MyTimer>()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins((ClickDetectorPlugin, MyTextPlugin, FlyPlugin))
        .add_systems(Startup, (setup_system, setup_ship_and_maus))
        .add_systems(
            PostStartup,
            (spawn_image_button_and_scoreboard, spawn_highscore),
        )
        .add_systems(
            Update,
            (
                rotate_system_flugi,
                button_timer_system,
                move_schnecke,
                listen_for_collision_events,
                update_scoreboard,
            ),
        )
        .add_systems(FixedUpdate, rotate_system)
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            force_update_from_transform_changes: true,
            ..default()
        })
        .add_systems(
            FixedUpdate,
            color_change_system.run_if(on_timer(std::time::Duration::from_secs(2))),
        )
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Create a cuboid mesh
    let box_mesh = meshes.add(Cuboid::new(0.5, 0.25, 0.25));

    // Create a red material
    let box_material = materials.add(Color::rgb(1.0, 0.2, 0.3));

    // Spawn the mesh entity with loaded materials
    commands
        .spawn(PbrBundle {
            mesh: box_mesh.clone(),
            material: box_material.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::from_rotation_y(45.0),
                scale: Vec3::new(3.0, 2.0, 1.0),
            },
            ..default()
        })
        .insert(Rotator { speed: 1.5 })
        .insert(Cubie)
        // Despawn an entity when clicked:
        .insert(PickableBundle::default())
        .insert(On::<Pointer<Move>>::target_component_mut::<Transform>(
            |_move, transform| {
                transform.scale = Vec3::new(3.0, 2.0, 1.0);
            },
        ))
        .insert(On::<Pointer<Click>>::target_commands_mut(
            |_click, target_commands| {
                target_commands.despawn();
            },
        ));

    // Add a camera and light source
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 2.0, 2.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 15.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle { ..default() });

    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/Windless Slopes.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

fn setup_ship_and_maus(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/Spaceship.glb#Scene0"),
            transform: Transform {
                translation: Vec3 {
                    x: -4.6,
                    y: -1.0,
                    z: -1.0,
                },
                rotation: default(),
                scale: Vec3 {
                    x: 0.2,
                    y: 0.2,
                    z: 0.2,
                },
            },
            ..default()
        },
        Rotator { speed: 1.0 },
        Spaceship,
        PickableBundle::default(),
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            transform.rotate_local_y(drag.delta.x / 100.0);
            transform.rotate_local_x(drag.delta.y / 100.0);
        }),
    ));

    commands.spawn(Camera2dBundle { ..default() });

    //spawn takes a tuple of components and adds them to the entity
    commands
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(100.),
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
            Rotator { speed: 1.0 },
            Mausi,
            //make Mausi dgraggable:
            PickableBundle::default(),
            On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
                transform.translation.x += drag.delta.x;
                transform.translation.y -= drag.delta.y;
            }),
            On::<Pointer<Move>>::run(my_funny_system),
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Sensor);

    //spawn Schnecke:
    let schnecke_pos = Vec3::new(-700.0, -300.0, 0.0);
    spawn_schnecke(commands, asset_server, schnecke_pos);
}

fn spawn_image_button_and_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        ImageBundle {
            image: asset_server.load("textures/click.png").into(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Default::default(),
                scale: Vec3::new(0.5, 0.5, 0.5),
            },
            style: Style {
                display: Display::Flex,
                justify_self: JustifySelf::Center,
                top: Val::Percent(50.0),
                ..Default::default()
            },
            ..Default::default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::run(button_callback_click),
    ));
    make_scoreboard(commands);
}

fn spawn_highscore(commands: Commands) {
    make_highscore(commands);
}

fn _make_beep(asset_server: &Res<AssetServer>, commands: &mut Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sounds/beep.mp3"),
        settings: PlaybackSettings::ONCE,
        //..default()
    });
}

fn button_callback_click(
    mut query: Query<&mut Transform, With<UiImage>>,
    mut timer: ResMut<MyTimer>,
    asset_server: Res<AssetServer>,
    commands: Commands,
) {
    println!("Button clicked!");
    // make_beep(&mut asset_server, &mut commands);

    let mut transform = query.single_mut();
    transform.scale = Vec3::new(0.3, 0.3, 0.3);

    timer.0.reset();

    //spawn Schnecke again:
    let schnecke_pos = Vec3::new(-700.0, -300.0, 0.0);
    spawn_schnecke(commands, asset_server, schnecke_pos);
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

fn button_timer_system(
    mut query: Query<&mut Transform, With<UiImage>>,
    time: Res<Time>,
    mut timer: ResMut<MyTimer>,
) {
    let mut transform = query.single_mut();

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        println!("Timer finished!");
        transform.scale = Vec3::new(0.5, 0.5, 0.5);
    }
}

fn my_funny_system(mut query: Query<&mut Transform, With<Cubie>>) {
    for mut transform in query.iter_mut() {
        transform.scale = Vec3::new(1.0, 2.0, 1.0);
    }
}

fn color_change_system(mut query: Query<&mut Sprite, With<Mausi>>) {
    for mut sprite in query.iter_mut() {
        let random_color = Color::rgb(
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        );
        sprite.color = random_color;
    }
}

fn rotate_system(
    time: Res<Time>,
    mut query: Query<(&Rotator, &mut Transform), Without<Spaceship>>,
) {
    for (rotator, mut transform) in query.iter_mut() {
        // Rotate the entity constantly along its z-axis (x- and y- must be done in rapier2d if needed)
        let rotation = Quat::from_rotation_z(rotator.speed * time.delta_seconds());
        transform.rotate(rotation);
    }
}

fn rotate_system_flugi(
    time: Res<Time>,
    mut query: Query<(&Rotator, &mut Transform), With<Spaceship>>,
    anim_state: Res<AnimationStateResource>,
) {
    if anim_state.moving {
        for (rotator, mut transform) in query.iter_mut() {
            // Rotate the entity constantly along its y-axis
            let rotation = Quat::from_rotation_y(rotator.speed * time.delta_seconds());
            transform.rotate(rotation);
        }
    }
}

fn listen_for_collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query: Query<Entity, With<Schnecke>>,
    mut scoreboard: ResMut<Scoreboard>,
) {
    // Create a vector to store entities involved in collisions
    let mut collided_entities: Vec<Entity> = Vec::new();

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                // Add the entities involved in the collision to the vector
                collided_entities.push(*entity1);
                collided_entities.push(*entity2);

                println!("CollisionEvent: Started");

                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/beep.mp3"),
                    settings: PlaybackSettings::ONCE,
                });

                // Iterate over the query to check which entities should be despawned
                for schnecke in query.iter() {
                    if collided_entities.contains(&schnecke) {
                        commands.entity(schnecke).despawn_recursive();
                    }
                }
                // update Scoreboard
                scoreboard.score += 1;
            }
            CollisionEvent::Stopped(_, _, _) => println!("CollisionEvent: Stopped"),
        }
    }
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}
