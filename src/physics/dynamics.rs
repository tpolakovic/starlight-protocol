use crate::physics::{
    gamma, Force, Mass, SpaceTimeAcc, SpaceTimePos, SpaceTimeVel, ThreeVector, TimeFactor,
};
use crate::{Player, TIME_STEP};
use bevy::prelude::*;

/// Updates the accelerations of entities forces.
pub(crate) fn update_acc(
    mut q_object: Query<(&mut SpaceTimeAcc, &Force, &SpaceTimeVel, &Mass), Without<Player>>,
    q_player: Query<(&Force, &Mass), With<Player>>,
) {
    let (p_f, p_m) = q_player.single();
    let acc_p = SpaceTimeAcc::from_force(p_m, p_f);
    for (mut acc, f, vel, m) in &mut q_object {
        let acc_o = SpaceTimeAcc::from_force(m, f);
        *acc = SpaceTimeAcc(acc_o.0 - acc_p.0).boost(&vel);
    }
}

/// Updates the velocities of entities.
pub(crate) fn update_vel(
    mut query: Query<(&SpaceTimeAcc, &mut SpaceTimeVel), Without<Player>>,
    time_factor: Res<TimeFactor>,
) {
    for (acc, mut vel) in &mut query {
        let new_vel = vel.0.r() + acc.0 * TIME_STEP * time_factor.0;
        *vel = SpaceTimeVel(Vec3::new(vel.t(), new_vel.x, new_vel.y));
    }
}

/// Updates the positions of objects that have kinematics enabled.
pub(crate) fn update_pos(
    mut query: Query<(&SpaceTimeVel, &mut SpaceTimePos)>,
    time_factor: Res<TimeFactor>,
) {
    for (vel, mut pos) in &mut query {
        let v = vel.0;
        let g = gamma(&v.r());
        let newpos = pos.0 + g * v * TIME_STEP * time_factor.0;
        *pos = SpaceTimePos(newpos);
    }
}
