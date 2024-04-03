use bevy::prelude::*;
use bevy_enoki::prelude::*;

#[derive(Component, Debug)]
pub struct ParticleEmitter;

pub fn emit_particle(
    mut cmd: Commands,
    mut materials: ResMut<Assets<SpriteParticle2dMaterial>>,
    server: Res<AssetServer>,
) {
    let texture_material = materials.add(
        // hframes and vframes define how the sprite sheet is divided for animations,
        // if you just want to bind a single texture, leave both at 1.
        SpriteParticle2dMaterial::new(server.load("textures/snail.png"), 1, 1),
    );
    cmd.spawn((
        ParticleSpawnerBundle {
            // load the effect configuration from a ron file
            effect: server.load("base.particle.ron"),
            // material: DEFAULT_MATERIAL,
            material: texture_material,
            ..default()
        },
        ParticleEmitter,
    ));
}

// https://lib.rs/crates/bevy_enoki
