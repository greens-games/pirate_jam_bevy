use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{plugin::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin, geometry::Collider, dynamics::RigidBody};
use player::systems::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, spawn_stuff)
        .run();
}


fn spawn_stuff(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {

    commands.spawn(Camera2dBundle{
        ..Default::default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::RegularPolygon {
            sides: 3,
            radius: 50.0
        })).into(),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        transform: Transform::from_xyz(100.0, 0.0, 0.0),
        ..default()
    })
    .insert(Collider::cuboid(25.0, 12.5)); 
}

