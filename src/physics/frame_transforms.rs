use super::{igamma, Contract, SpaceTimeObject, Vel};
use bevy::{
    math::{Mat3A, Vec3A},
    prelude::*,
};

/// Length-contracts every spacetime object relative to player frame.
pub(crate) fn redraw_in_player_frame(
    mut q: Query<(&Vel, &mut GlobalTransform), With<SpaceTimeObject>>,
) {
    for (Vel(v), mut gt) in q.iter_mut() {
        if v.length() > 0. {
            let mut ga = gt.affine();
            let g = igamma(&v);
            ga.translation = ga.translation.contract(v);
            let v = v.normalize();
            let col1 = Vec3A::new(1. + (g - 1.) * v.x * v.x, (g - 1.) * v.x * v.y, 0.);
            let col2 = Vec3A::new((g - 1.) * v.x * v.y, 1. + (g - 1.) * v.y * v.y, 0.);
            let col3 = Vec3A::Z;
            let smat = Mat3A::from_cols(col1, col2, col3);
            ga.matrix3 = smat * ga.matrix3;
            *gt = ga.into();
        }
    }
}
