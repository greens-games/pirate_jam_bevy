use bevy::{prelude::*};
use bevy_rapier2d::{geometry::Collider, control::KinematicCharacterController, dynamics::Velocity};

use super::components::Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

fn spawn_player(
    mut commands: Commands
) {

    commands.spawn(SpriteBundle {
        sprite: Sprite{
            custom_size: Some(Vec2::new(16.0, 16.0)),
            color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Collider::cuboid(8.0, 8.0))
    .insert(KinematicCharacterController {
        translation: Some(Vec2::new(-50.0,0.0)),
        ..Default::default()
    });
}
fn move_player(
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>
) {

    let mut transform:Mut<'_, KinematicCharacterController> = player_query.get_single_mut().unwrap();

    let speed:f32 = 100.0;
    let mut velocity:Vec2 = Vec2::ZERO;

    if input.pressed(KeyCode::D) {
        velocity.x += speed * time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        velocity.x -= speed * time.delta_seconds();
    }
    
    if input.pressed(KeyCode::W) {
        velocity.y += speed * time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        velocity.y -= speed * time.delta_seconds();
    }

    transform.translation = Some(velocity);

}


