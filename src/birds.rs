use crate::physics::{Gravity, Velocity, VelocityRotator};
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
struct Bird;

#[derive(Resource)]
struct JumpForce(f32);

#[derive(Resource)]
struct DelayTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_rotate(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity, &VelocityRotator), With<TextureAtlas>>,
) {
    for (mut transform, velocity, velocity_rotator) in query.iter_mut() {
        let max_r_v = velocity_rotator.max_rotate_vel;

        if velocity.0.y >= 0.0 {
            // Target rotation
            let target_rotation = Quat::from_rotation_z(
                max_r_v * velocity_rotator.rotation_angle * time.delta_seconds(),
            );

            // Smooth interpolation towards the target rotation
            transform.rotation = transform
                .rotation
                .lerp(target_rotation, velocity_rotator.lerp_factor);
        } else {
            // Target rotation
            let target_rotation = Quat::from_rotation_z(
                -max_r_v * velocity_rotator.rotation_angle * time.delta_seconds(),
            );

            // Smooth interpolation towards the target rotation
            transform.rotation = transform
                .rotation
                .lerp(target_rotation, velocity_rotator.lerp_factor);
        }
    }
}

fn animate_physic(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<TextureAtlas>>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.y += velocity.0.y * time.delta_seconds();
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn spawn_bird(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = assets_server.load("bird.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 2, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 3 };
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(2.0)),
            texture,
            ..Default::default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        Velocity(Vec2 { x: 0.0, y: 10.0 }),
        Gravity(10.0 * 30.0),
        VelocityRotator {
            rotation_angle: PI / 4.0,
            lerp_factor: 1.0,
            max_rotate_vel: 20.0,
        },
        Bird,
    ));
}

fn player_input(
    time: Res<Time>,
    mut timer: ResMut<DelayTimer>,
    jump_force: Res<JumpForce>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity>,
) {
    if timer.0.tick(time.delta()).finished() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            info!("'SPACE' currently pressed");
            // Adjust veolcity when press  space
            query.iter_mut().for_each(|mut vel| {
                vel.0.y = jump_force.0 * time.delta_seconds() + 400.0;
            })
        }
    }
}

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird)
            .insert_resource(DelayTimer(Timer::from_seconds(0.02, TimerMode::Repeating)))
            .insert_resource(JumpForce(300.0))
            .add_systems(Update, animate_rotate)
            .add_systems(Update, animate_sprite)
            .add_systems(Update, animate_physic)
            .add_systems(Update, player_input);
    }
}
