use bevy::{prelude::*, utils::FloatOrd};
use bevy_inspector_egui::WorldInspectorPlugin;
use bullet::*;
use target::*;
use tower::*;

mod bullet;
mod target;
mod tower;

pub const WIDTH: f32 = 1280.0;
pub const HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Tower Defence Tutorial".to_string(),
                resizable: false,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .register_type::<Lifetime>()
        .register_type::<Target>()
        .register_type::<Bullet>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_system(spawn_enemy)
        .add_plugin(TowerPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(BulletPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.36, 0.39, 0.33).into()),
            ..default()
        })
        .insert(Name::new("Plane"))
        .insert(SpawnRate {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        });
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(175.0 / 255.0, 144.0 / 255.0, 112.0 / 255.0).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            bullet_offset: Vec3 {
                x: 0.0,
                y: 0.25,
                z: 0.55,
            },
        })
        .insert(Name::new("Tower"));
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 2000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}

fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut planes: Query<&mut SpawnRate>,
) {
    for mut plane in &mut planes {
        if plane.timer.tick(time.delta()).just_finished() {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                    material: materials.add(Color::rgb(0.7, 0.3, 0.3).into()),
                    transform: Transform::from_xyz(-2.2, 0.2, 2.2),
                    ..default()
                })
                .insert(Target { speed: 0.3 })
                .insert(Health { value: 3 })
                .insert(Name::new("Target"));
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct SpawnRate {
    timer: Timer,
}
