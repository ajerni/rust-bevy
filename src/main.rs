//! # ECS (Entity, Component, System)
//! 
//! Bundles of components are spawned to the world as entities and are controlled
//! by systems that run based on schedules.
//! 
//! ![ECS overview](https://csherratt.github.io/blog/posts/specs-and-legion/DraggedImage.png)
//! 

mod button;
mod controls;
mod db;
mod fly_plugin;
mod gamestate;
mod particle;
mod paused;
mod schnecke;
mod scoreboard;
mod texts;
mod timers;
pub mod ui;

use bevy_mod_picking::events::{Click, Drag, Move, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::DefaultPickingPlugins;

use crate::button::*;
use crate::db::*;
use crate::gamestate::*;
use crate::particle::*;
use crate::paused::*;
use crate::schnecke::*;
use crate::scoreboard::*;
use crate::ui::*;
use bevy_enoki::prelude::*;
use controls::*;
use fly_plugin::FlyPlugin;
use texts::MyTextPlugin;
use timers::MyTimer;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use bevy_rapier2d::prelude::*;


#[derive(Component, Debug)]
/// Component used to set the speed of the rotation
/// 
/// # Simply adding this component to an entity will rotate it at the given speed.
pub struct Rotator {
    speed: f32,
}

#[derive(Component)]
/// Marker struct for the backgound music
struct MyBackgroundMusic;

///Setting up the App
/// 
/// # Here is the place to:
/// - insert resources
/// - init resources
/// - init states
/// - add events
/// - add plugins
/// - add systems
/// 
fn main() {
    App::new()
        .insert_resource(AnimationStateResource { moving: false })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 1.0)))
        .insert_resource(Scoreboard {
            score: 0,
            highscore: 0,
            player_name: String::from("Andi"),
            highscore_holder: String::from("Andi"),
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            physics_pipeline_active: true,
            force_update_from_transform_changes: true,
            ..default()
        })
        .init_resource::<MyTimer>() // init initializes with Default value
        .init_state::<GameState>()
        .init_state::<SchneckenEmitterState>()
        .add_event::<GetDataEvent>() //my own event that triggers load from DB
        .add_event::<UpdateDataEvent>() //my own event that triggers update of highscore in DB
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins) // for drag&drop and hover(move)
        .add_plugins((
            ClickDetectorPlugin,
            MyTextPlugin,
            FlyPlugin,
            PausePlugin,
            UiPlugin,
            DataPlugin,
        ))
        .add_plugins(EnokiPlugin) //for particle emmitters
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(PostStartup, init_state_for_camera)
        .add_systems(
            Startup,
            (setup_system, setup_ship_and_maus).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Startup,
            (spawn_button, spawn_scoreboard, spawn_highscore)
                .chain()
                .after(setup_ship_and_maus)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnEnter(SchneckenEmitterState::Emitting),
            spawn_schnecke_emitter,
        )
        .add_systems(OnExit(SchneckenEmitterState::Emitting), destroy_emitter)
        .add_systems(OnEnter(GameState::Paused), pause_music_toggle)
        .add_systems(OnExit(GameState::Paused), pause_music_toggle)
        .add_systems(
            Update,
            (
                rotate_system_flugi,
                move_schnecke,
                listen_for_collision_events,
                update_scoreboard,
                init_highscore_holder,
                update_highscore,
                check_score_changed,
                button_timer_system,
            )
                .chain()
                .run_if(in_state(GameState::Playing)), //chain ensures to add the systems in the given order
        )
        .add_systems(
            FixedUpdate,
            rotate_system.run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            FixedUpdate,
            // 'or_else()' false/true AND 'and_then()' true/true
            color_change_system.run_if(on_timer(std::time::Duration::from_secs(2))),
        )
        .add_systems(OnEnter(GameState::Playing), show_button)
        .add_systems(OnExit(GameState::Playing), hide_button)
        .add_systems(OnEnter(GameState::Playing), show_score)
        .add_systems(OnExit(GameState::Playing), hide_score)
        .add_systems(OnEnter(GameState::Playing), show_highscore)
        .add_systems(OnExit(GameState::Playing), hide_highscore)
        .run();
}

/// using Visibility to show/hide UI elements
fn show_button(mut menu: Query<&mut Visibility, With<UiImage>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}

/// using Visibility to show/hide UI elements
fn hide_button(mut menu: Query<&mut Visibility, With<UiImage>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}

/// using Visibility to show/hide UI elements
fn show_score(mut menu: Query<&mut Visibility, With<ScoreboardUi>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}

/// using Visibility to show/hide UI elements
fn hide_score(mut menu: Query<&mut Visibility, With<ScoreboardUi>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}

/// using Visibility to show/hide UI elements
fn show_highscore(mut menu: Query<&mut Visibility, With<HighscoreUi>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Visible;
}

/// using Visibility to show/hide UI elements
fn hide_highscore(mut menu: Query<&mut Visibility, With<HighscoreUi>>) {
    let mut menu = menu.single_mut();
    *menu = Visibility::Hidden;
}

/// spawns a MaterialMeshBundel, a camera, two light sources and the background music
fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut get_highscore_event: EventWriter<GetDataEvent>,
) {
    // Get Highscore from DB (whenever this Event is sent it triggers update from DB)
    get_highscore_event.send_default();

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
                scale: Vec3::new(2.5, 2.0, 1.0),
            },
            ..default()
        })
        .insert(Rotator { speed: 1.5 })
        .insert(Cubie)
        .insert(On::<Pointer<Move>>::target_component_mut::<Transform>(
            |_move, transform| {
                transform.scale = Vec3::new(2.5, 2.0, 1.0);
            },
        ))
        // Despawn an entity when clicked:
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

    // Add a background music
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            settings: PlaybackSettings::LOOP,
        },
        MyBackgroundMusic,
    ));
}

/// spawns spaceship and maus sprite (and 2d camera)
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
        On::<Pointer<Drag>>::target_component_mut::<Transform>(move |drag, transform| {
            transform.rotate_local_y(drag.delta.x / 100.0);
            transform.rotate_local_x(drag.delta.y / 100.0);
        }),
    ));

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 2,
            ..default()
        },
        ..default()
    });

    //spawn takes a tuple of components and creates an entity
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
            On::<Pointer<Drag>>::target_component_mut::<Transform>(move |drag, transform| {
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

/// spawns the img button
fn spawn_button(commands: Commands, asset_server: Res<AssetServer>) {
    make_button(commands, asset_server);
}

/// spawns the Scoreboard UI
fn spawn_scoreboard(commands: Commands) {
    make_scoreboard(commands);
}

/// spawns the Highscore UI
fn spawn_highscore(commands: Commands) {
    make_highscore(commands);
}

/// spawn Schnecken-Emitter (runs on state change [crate::gamestate::SchneckenEmitterState]):
fn spawn_schnecke_emitter(
    commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<SpriteParticle2dMaterial>>,
) {
    emit_particle(commands, materials, asset_server)
}

//SYSTEMS:

/// system to move the Schnecke from left to right
fn move_schnecke(mut query: Query<&mut Transform, With<Schnecke>>, time: Res<Time>) {
    let speed = 90.0;
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

/// using the MyTimer Resource to animate the button
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

/// system to change the scale of the cube
fn my_funny_system(mut query: Query<&mut Transform, With<Cubie>>) {
    for mut transform in query.iter_mut() {
        transform.scale = Vec3::new(1.0, 2.0, 1.0);
    }
}

/// system to change the color of the mouse every 2 seconds
/// 
/// see run condition on App.add_systems(): `color_change_system.run_if(on_timer(std::time::Duration::from_secs(2)))``
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

/// using the speed set in the [Rotator]
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

/// using the speed set in the [Rotator]
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

/// listening for colision events porvided by rapier2d
fn listen_for_collision_events(
    //CollisionEvent comes from rapier2d (Alternative könnte bevy_xpbd_2d sein)
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

/// # system that constantly sets the Scoreboard UIs text section to the score value of the Scoreboard Resource
fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
    let mut text = query.single_mut();
    text.sections[1].value = scoreboard.score.to_string();
}

/// system that initiates the highscore display
fn init_highscore_holder(
    scoreboard: Res<Scoreboard>,
    mut query: Query<&mut Text, With<HighscoreUi>>,
) {
    let mut text = query.single_mut();
    text.sections[3].value = scoreboard.highscore_holder.clone();
}

/// system that updates the highscore in the Scoreboard Resource and on the Database
/// 
/// [crate::controls::UpdateDataEvent] is used to trigger the database update in `db.rs``
fn update_highscore(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scoreboard: ResMut<Scoreboard>,
    mut query: Query<&mut Text, With<HighscoreUi>>,
    mut update_highscore_event: EventWriter<UpdateDataEvent>,
) {
    let mut text = query.single_mut();

    if scoreboard.score > scoreboard.highscore {
        scoreboard.highscore = scoreboard.score;
        scoreboard.highscore_holder = scoreboard.player_name.clone();
        text.sections[3].value = scoreboard.highscore_holder.clone();
        //updating the highscore in the DB:
        update_highscore_event.send_default();
    }

    text.sections[1].value = scoreboard.highscore.to_string();

    //Reset functions for Score and Highscore:
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        scoreboard.score = 0;
    }
    if keyboard_input.just_pressed(KeyCode::KeyH) {
        scoreboard.highscore = 0;
    }
}

/// # system that checks for the "winning state" which is a score equalt to 3
/// 
/// on winning state the state [crate::gamestate::SchneckenEmitterState] triggers the spawn of the particle emitter
fn check_score_changed(
    my_res: Res<Scoreboard>,
    mut next_state: ResMut<NextState<SchneckenEmitterState>>,
) {
    if my_res.score == 3 {
        next_state.set(SchneckenEmitterState::Emitting);
    } else {
        //Destroy the schnecken emitter if existing
        next_state.set(SchneckenEmitterState::NotEmitting);
    }
}

// Option with Option :-) - in case the state might not be initiallized yet or to queue the transition
// fn check_score_changed(
//     my_res: Res<Scoreboard>,
//     state : Res<State<SchneckenEmitterState>>,
//     next_state: Option<ResMut<NextState<SchneckenEmitterState>>>,
// )
// {
//     if my_res.score == 3 {
//         let my_state = state.get();
//         if my_state == &SchneckenEmitterState::NotEmitting {
//             println!("You won! Congratulations!");
//             println!("State before transition: {:?}", my_state);

//             Some(next_state.unwrap().set(SchneckenEmitterState::Emitting));
//         }
//     }
// }

/// system that stops the emitter (using OnExit schedule)
/// 
/// # see App setting: `.add_systems(OnExit(SchneckenEmitterState::Emitting), destroy_emitter)`
fn destroy_emitter(mut commands: Commands, query: Query<Entity, With<ParticleEmitter>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// system that toggles the music to be playing or pause (uses pattern matching)
fn pause_music_toggle(music_controller: Query<&AudioSink, With<MyBackgroundMusic>>) {
    // pattern matching on Result<T, E>
    if let Ok(sink) = music_controller.get_single() {
        sink.toggle();
    }
}

/// system that initiallises the Menu variant of the GameState ([crate::gamestate::GameState])
fn init_state_for_camera(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Menu);
}
