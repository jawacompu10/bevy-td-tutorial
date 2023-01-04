use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(tower_shooting);
    }
}

fn tower_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (entity, mut tower, tower_transform) in &mut towers {
        if tower.shooting_timer.tick(time.delta()).just_finished() {
            commands.entity(entity).with_children(|commands| {
                let bullet_spawn = tower_transform.translation() + tower.bullet_offset;
                let direction = targets
                    .iter()
                    .min_by_key(|target_transform| {
                        FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                    })
                    .map(|closest_target| closest_target.translation() - bullet_spawn);

                if let Some(direction) = direction {
                    commands
                        .spawn(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::UVSphere {
                                radius: 0.1,
                                ..default()
                            })),
                            material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
                            transform: Transform::from_translation(tower.bullet_offset),
                            ..default()
                        })
                        .insert(Bullet {
                            speed: 2.0,
                            direction,
                        })
                        .insert(Name::new("Bullet"))
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1.0, TimerMode::Once),
                        });
                }
            });
        }
    }
}
