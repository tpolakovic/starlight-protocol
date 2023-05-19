use std::f32::consts::FRAC_2_PI;

use bevy::{
    math::{Mat3A, Vec3A},
    prelude::*,
};

use super::spacetime::{igamma, Contract, RealGlobalTransform, SpaceTimeObject, Velocity};

fn scale_matrix(v: &Vec2, g: f32) -> Mat3A {
    let vn = v.normalize();
    let col1 = Vec3A::new(1. + (g - 1.) * vn.x * vn.x, (g - 1.) * vn.x * vn.y, 0.);
    let col2 = Vec3A::new((g - 1.) * vn.x * vn.y, 1. + (g - 1.) * vn.y * vn.y, 0.);
    let col3 = Vec3A::Z;
    Mat3A::from_cols(col1, col2, col3)
}

/// Length-contracts every spacetime object relative to player frame.
pub(crate) fn redraw_in_player_frame(
    mut q: Query<
        (
            &Velocity,
            &mut GlobalTransform,
            &mut RealGlobalTransform,
            Ref<Transform>,
        ),
        With<SpaceTimeObject>,
    >,
) {
    for (Velocity(velocity), mut global_transform, mut real_global_transform, transform) in
        q.iter_mut()
    {
        if transform.is_changed() {
            real_global_transform.0 = *global_transform;
            if velocity.length() > 0. {
                let mut global_affine = global_transform.affine();
                let g = igamma(&velocity);

                let angle = global_affine
                    .translation
                    .truncate()
                    .angle_between(*velocity);
                let abberated_angle = f32::acos(
                    (f32::cos(angle) - velocity.length())
                        / (1. - f32::cos(angle) * velocity.length()),
                ) * angle.signum();

                global_affine.translation = Vec2::from_angle(angle - abberated_angle)
                    .rotate(global_affine.translation.truncate())
                    .extend(global_affine.translation.z)
                    .into();
                global_affine.translation = global_affine.translation.contract(velocity);

                let smat = scale_matrix(velocity, g);
                let smat2 = scale_matrix(
                    &velocity.perp(),
                    1. - f32::asin(velocity.length()) * FRAC_2_PI,
                );

                global_affine.matrix3 = smat2
                    * Mat3A::from_rotation_z(angle - abberated_angle)
                    * smat
                    * global_affine.matrix3;

                *global_transform = global_affine.into();
            }
        }
    }
}
