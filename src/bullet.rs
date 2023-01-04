use crate::*;

fn despawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in &mut bullets {
        if lifetime.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_bullets)
            .add_system(bullet_collision)
            .add_system(despawn_bullet);
    }
}

fn move_bullets(mut bullets: Query<(&Bullet, &mut Transform)>, time: Res<Time>) {
    for (bullet, mut transform) in &mut bullets {
        transform.translation += bullet.direction.normalize() * bullet.speed * time.delta_seconds();
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &Transform), With<Target>>,
) {
    for (bullet, b_transform) in &bullets {
        for (mut health, t_transform) in &mut targets {
            if Vec3::distance(b_transform.translation(), t_transform.translation) < 0.3 {
                commands.entity(bullet).despawn_recursive();
                health.value -= 1;
                break;
            }
        }
    }
}
