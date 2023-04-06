use std::f32::consts::FRAC_2_PI;

use super::{igamma, Contract, SpaceTimeObject, Vel};
use bevy::{
    math::{Mat3A, Vec3A},
    prelude::*,
};

fn scale_matrix(v: &Vec2, g: f32) -> Mat3A {
    let vn = v.normalize();
    let col1 = Vec3A::new(1. + (g - 1.) * vn.x * vn.x, (g - 1.) * vn.x * vn.y, 0.);
    let col2 = Vec3A::new((g - 1.) * vn.x * vn.y, 1. + (g - 1.) * vn.y * vn.y, 0.);
    let col3 = Vec3A::Z;
    Mat3A::from_cols(col1, col2, col3)
}

/// Length-contracts every spacetime object relative to player frame.
pub(crate) fn redraw_in_player_frame(
    mut q: Query<(&Vel, &mut GlobalTransform), With<SpaceTimeObject>>,
) {
    for (Vel(v), mut gt) in q.iter_mut() {
        if v.length() > 0. {
            let mut ga = gt.affine();
            let g = igamma(&v);
            let a0 = ga.translation.truncate().angle_between(*v);
            let a1 = f32::acos((f32::cos(a0) - v.length()) / (1. - f32::cos(a0) * v.length()))
                * a0.signum();
            ga.translation = Vec2::from_angle(a0 - a1)
                .rotate(ga.translation.truncate())
                .extend(ga.translation.z)
                .into();
            ga.translation = ga.translation.contract(v);
            let smat = scale_matrix(v, g);
            let smat2 = scale_matrix(&v.perp(), 1. - f32::asin(v.length()) * FRAC_2_PI);
            ga.matrix3 = smat2 * Mat3A::from_rotation_z(a0 - a1) * smat * ga.matrix3;
            *gt = ga.into();
        }
    }
}
