use bevy::prelude::*;
use std::ops::Neg;

use super::{gamma, Acceleration, Force, InverseMass, LocalTime, Player, TimeFactor, Velocity};

/// Updates the accelerations of entities' forces.
pub(crate) fn update_acceleration(
    mut query_object: Query<(&mut Acceleration, &Force, &Velocity, &InverseMass), Without<Player>>,
    query_player: Query<(&Force, &InverseMass), With<Player>>,
) {
    let (player_force, player_inverse_mass) = query_player.single();
    let player_acceleration = Acceleration::from_force(player_inverse_mass, player_force);
    for (mut total_acceleration, force, Velocity(velocity), inverse_mass) in &mut query_object {
        let object_acceleration = Acceleration::from_force(inverse_mass, force);
        *total_acceleration =
            (object_acceleration - player_acceleration.boost(&velocity.neg())).boost(&velocity);
    }
}

/// Updates the velocities of entities.
pub(crate) fn update_velocity(
    mut query: Query<(&Acceleration, &mut Velocity), Without<Player>>,
    time_factor: Res<TimeFactor>,
    dt: Res<FixedTime>,
) {
    for (&Acceleration(acceleration), mut velocity) in &mut query {
        velocity.0 += acceleration * dt.period.as_secs_f32() * time_factor.0;
    }
}

/// Updates the positions of objects.
pub(crate) fn update_position(
    mut query: Query<(&Velocity, &mut LocalTime, &mut Transform)>,
    time_factor: Res<TimeFactor>,
    dt: Res<FixedTime>,
) {
    for (Velocity(velocity), mut time, mut transform) in &mut query {
        let g = gamma(velocity);
        time.0 += (g * time_factor.0 * dt.period.as_secs_f32()) as f64;
        let position_delta =
            Vec3::new(g * velocity.x, g * velocity.y, 0.) * dt.period.as_secs_f32() * time_factor.0;
        transform.translation += position_delta;
    }
}

/// Clears all forces at the end of the simulation frame.
pub(crate) fn clear_forces(mut query: Query<&mut Force>) {
    for mut force in &mut query.iter_mut() {
        *force = Force::ZERO;
    }
}
