use background::BackGroundPlugin;
use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use birds::BirdPlugin;
use physics::PhysicsPlugin;

mod background;
mod birds;
mod physics;

#[derive(Component)]
struct MyCameraMaker;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            Wireframe2dPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BirdPlugin)
        .add_plugins(BackGroundPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, setup_camera)
        .insert_resource(ClearColor(Color::srgb(0.34, 0.75, 0.79)))
        .add_systems(Update, toggle_wireframe)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(300.0, 10.0, 0.0),
            ..Default::default()
        },
        MyCameraMaker,
    ));
}

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(SpriteBundle {
//         texture: asset_server.load("bird.png"),
//         ..Default::default()
//     });
// }

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
