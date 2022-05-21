use bevy::{
    prelude::*,
    render::{options::WgpuOptions, render_resource::WgpuLimits},
};
use heron::prelude::*;

fn main() {
    App::new()
        .insert_resource(WgpuOptions {
            limits: WgpuLimits {
                max_texture_array_layers: 2048,
                ..Default::default()
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Gravity::from(Vec3::new(0.0, -500., 0.0)))
        .add_startup_system(startup_system)
        .add_system(jetpack)
        .run();
}

#[derive(Component, Clone, Debug, Default, Bundle)]
pub struct ColliderBundle {
    pub collider: CollisionShape,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub rotation_constraints: RotationConstraints,
}

#[derive(Component, Clone, Default)]
pub struct Player;

fn startup_system(mut commands: Commands) {
    println!("ayooo");
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(ColliderBundle {
            collider: CollisionShape::Capsule {
                half_segment: 20.0,
                radius: 20.0,
            },
            rigid_body: RigidBody::Dynamic,
            rotation_constraints: RotationConstraints::lock(),
            ..Default::default()
        })
        .insert(Player)
        .insert(PhysicMaterial {
            restitution: 0.5,
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        });

    commands
        .spawn_bundle(ColliderBundle {
            collider: CollisionShape::Cuboid {
                half_extends: Vec3::new(90., 5., 0.),
                border_radius: None,
            },
            rigid_body: RigidBody::Static,
            rotation_constraints: RotationConstraints::lock(),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., 80., 0.),
            ..Default::default()
        });

    commands
        .spawn_bundle(ColliderBundle {
            collider: CollisionShape::Cuboid {
                half_extends: Vec3::new(90., 5., 0.),
                border_radius: None,
            },
            rigid_body: RigidBody::Static,
            rotation_constraints: RotationConstraints::lock(),
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            transform: Transform::from_xyz(0., -80., 0.),
            ..Default::default()
        });
}

fn jetpack(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        let quat_z = transform.rotation.z;
        let rads: f32;
        if quat_z < 0.0 {
            rads = transform.rotation.to_axis_angle().1 * -1.;
        } else {
            rads = transform.rotation.to_axis_angle().1;
        }

        let direction = Vec2::new(rads.sin() * -1.0, rads.cos());

        if input.pressed(KeyCode::Space) {
            velocity.linear.y += direction.y * 10.;
            velocity.linear.x += direction.x * 10.;

            if velocity.linear.y > 200.0 {
                velocity.linear.y = 200.0;
            }
            if velocity.linear.y < -200.0 {
                velocity.linear.y = -200.0;
            }

            if velocity.linear.x > 200.0 {
                velocity.linear.x = 200.0;
            }
            if velocity.linear.x < -200.0 {
                velocity.linear.x = -200.0;
            }
        }
        println!("{}", velocity.linear.y);
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            transform.rotate(Quat::from_rotation_z(0.01));
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            transform.rotate(Quat::from_rotation_z(-0.01));
        }
    }
}
