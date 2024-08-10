use bevy::prelude::*;
use rand::{thread_rng,Rng};


use crate::physics;
use  physics::Velocity; 

#[derive(Component)]
pub struct Cloud;

pub struct BackGroundPlugin;

impl Plugin for BackGroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, cloud_spawn)
            .add_systems(Update, moving_background);
    }
}



#[derive(Resource)]
pub struct SpawnTimer(Timer);

fn cloud_spawn(
    mut commands: Commands,
    time: Res<Time>,
    mut cloud_timer: ResMut<SpawnTimer>,
    asset_server: Res<AssetServer>,
    //mut material: ResMut<Assets<ColorMaterial>>,

) {
    let mut rng = thread_rng();

    let cloud_texture_handle = if rng.gen_bool(0.5) {
        asset_server.load("cloud_1.png")
    } else {
        asset_server.load("cloud_2.png")
    };

    cloud_timer.0.tick(time.delta());
    if cloud_timer.0.finished() {
        commands.spawn((
            SpriteBundle {
                texture: cloud_texture_handle,
                transform : Transform { 
                    translation: Vec3::new(
                        1000.0  + 30.0,
                        rng.gen_range(0.0..200.0),
                        2.0,
                    ),
                    scale: Vec3::splat(rng.gen_range(2.0..4.0)),
                    ..Default::default()
                 },
                 ..Default::default()
            },
            Velocity(Vec2 { x: rng.gen_range(-70.0..-40.0), y: rng.gen_range(-50.0..50.0) }),
            Cloud,
        ));
    }


}

fn moving_background(
    time: Res<Time>,
   mut  query: Query<(&mut Transform, &Velocity), With<Cloud>>,
) {
    for (mut transform, vel) in &mut query  {
        transform.translation.x += vel.0.x*time.delta_seconds();
       // transform.translation.y = vel.0.y*time.delta_seconds();
    }
}

fn clean_up() {
    todo!();
}