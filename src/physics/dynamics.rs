use bevy::prelude::*;
use std::ops::Neg;

use super::{gamma, Acc, Force, LocalTime, Mass, Player, TimeFactor, Vel};

/// Updates the accelerations of entities forces.
pub(crate) fn update_acc(
    mut q_object: Query<(&mut Acc, &Force, &Vel, &Mass), Without<Player>>,
    q_player: Query<(&Force, &Mass), With<Player>>,
) {
    let (p_f, p_m) = q_player.single();
    let acc_p = Acc::from_force(p_m, p_f);
    for (mut acc, f, Vel(v), m) in &mut q_object {
        // let vel = v.r();
        let acc_o = Acc::from_force(m, f);
        *acc = (acc_o - acc_p.boost(&v.neg())).boost(&v);
    }
}

/// Updates the velocities of entities.
pub(crate) fn update_vel(
    mut query: Query<(&Acc, &mut Vel), Without<Player>>,
    time_factor: Res<TimeFactor>,
    dt: Res<FixedTime>,
) {
    for (&Acc(a), mut vel) in query.iter_mut() {
        vel.0 += a * dt.period.as_secs_f32() * time_factor.0;
    }
}

/// Updates the positions of objects that have kinematics enabled.
pub(crate) fn update_pos(
    mut query: Query<(&Vel, &mut LocalTime, &mut Transform)>,
    time_factor: Res<TimeFactor>,
    dt: Res<FixedTime>,
) {
    for (Vel(v), mut t, mut tr) in &mut query {
        let g = gamma(&v);
        t.0 += g;
        let p = Vec3::new(g * v.x, g * v.y, 0.) * dt.period.as_secs_f32() * time_factor.0;
        tr.translation += p;
    }
}
