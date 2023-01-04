use crate::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_targets).add_system(target_death);
    }
}

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (target, health) in &targets {
        if health.value <= 0 {
            commands.entity(target).despawn_recursive();
        }
    }
}
