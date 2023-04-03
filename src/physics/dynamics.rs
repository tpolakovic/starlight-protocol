use crate::physics::{
    gamma, Acc, Force, Mass, SpaceTimePos, SpaceTimeVel, ThreeVector, TimeFactor,
};
use crate::Player;
use bevy::prelude::*;
use std::ops::Neg;

/// Updates the accelerations of entities forces.
pub(crate) fn update_acc(
    mut q_object: Query<(&mut Acc, &Force, &SpaceTimeVel, &Mass), Without<Player>>,
    q_player: Query<(&Force, &Mass), With<Player>>,
) {
    let (p_f, p_m) = q_player.single();
    let acc_p = Acc::from_force(p_m, p_f);
    for (mut acc, f, vel, m) in &mut q_object {
        let acc_o = Acc::from_force(m, f);
        *acc = (acc_o - acc_p.boost(&vel.neg())).boost(vel);
    }
}

/// Updates the velocities of entities.
pub(crate) fn update_vel(
    mut query: Query<(&Acc, &mut SpaceTimeVel), Without<Player>>,
    time_factor: Res<TimeFactor>,
    dt: Res<FixedTime>,
) {
    for (acc, mut vel) in &mut query {
        let new_vel = vel.r() + acc.r() * dt.period.as_secs_f32() * time_factor.0;
        *vel = SpaceTimeVel(Vec3::new(vel.t(), new_vel.x, new_vel.y));
    }
}

/// Updates the positions of objects that have kinematics enabled.
pub(crate) fn update_pos(
    mut query: Query<(&SpaceTimeVel, &mut SpaceTimePos)>,
    time_factor: Res<TimeFactor>,
    dt: Res<FixedTime>,
) {
    for (vel, mut pos) in &mut query {
        let g = gamma(&vel);
        let v = vel.0;
        let newpos = pos.0 + g * v * dt.period.as_secs_f32() * time_factor.0;
        *pos = SpaceTimePos(newpos);
    }
}
