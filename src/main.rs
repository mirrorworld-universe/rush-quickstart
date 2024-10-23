use bevy::{
    color::palettes::css::BLUE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rush_ecs_core::blueprint::{Component, ComponentValue};
use rush_ecs_sdk::bevy::BevySDK;

const SPEED: f32 = 1000.0;

#[derive(Component)]
struct Player;

#[derive(Clone, Component)]
struct Position(f32, f32);

fn main() {
    // load manifest onchain
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (input, update).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // spawn player
    let player_square = Mesh2dHandle(meshes.add(Rectangle::new(50.0, 50.0)));
    commands.spawn((
        Player,
        Position(0., 0.),
        MaterialMesh2dBundle {
            mesh: player_square.clone(),
            material: materials.add(Color::from(BLUE)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ));
}

// SET STATE HERE
fn input(
    time: Res<Time>,
    mut player_query: Query<&mut Position, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    // check for keypress
    for mut position in &mut player_query {
        if keys.pressed(KeyCode::KeyW) {
            position.1 += SPEED * time.delta_seconds();
        } else if keys.pressed(KeyCode::KeyS) {
            position.1 -= SPEED * time.delta_seconds();
        }

        if keys.pressed(KeyCode::KeyA) {
            position.0 -= SPEED * time.delta_seconds();
        } else if keys.pressed(KeyCode::KeyD) {
            position.0 += SPEED * time.delta_seconds();
        }
    }
}

// GET STATE HERE
fn update(mut player_query: Query<(&mut Transform, &Position), With<Player>>) {
    for (mut transform, position) in &mut player_query {
        transform.translation.x = position.0;
        transform.translation.y = position.1;
    }
}
