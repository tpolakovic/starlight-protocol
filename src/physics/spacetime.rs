use std::ops::{Neg, Sub};

use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

// A vector that transforms correctly under Lorentz transformations in 2+1D spacetime.
pub(crate) trait ThreeVector {
    fn t(&self) -> f32;
    fn r(&self) -> Vec2;
}

impl ThreeVector for Vec3 {
    fn t(&self) -> f32 {
        self.x
    }

    fn r(&self) -> Vec2 {
        Vec2::new(self.y, self.z)
    }
}

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct SpaceTimePos(pub Vec3);

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct SpaceTimeVel(pub Vec3);

impl Default for SpaceTimeVel {
    fn default() -> Self {
        SpaceTimeVel(Vec3::new(1., 0., 0.))
    }
}

impl Neg for &SpaceTimeVel {
    type Output = SpaceTimeVel;

    fn neg(self) -> Self::Output {
        SpaceTimeVel(-self.0)
    }
}

#[inline]
/// Inverse of Lorentz factor.
pub(crate) fn igamma(v: &SpaceTimeVel) -> f32 {
    let v = v.r();
    f32::sqrt(1. - v.length_squared())
}

#[inline]
/// Lorentz factor.
pub(crate) fn gamma(v: &SpaceTimeVel) -> f32 {
    igamma(v).recip()
}

macro_rules! sp_impl {
    ($spo:ty) => {
        impl std::ops::Deref for $spo {
            type Target = Vec3;

            fn deref(&self) -> &Vec3 {
                &self.0
            }
        }

        impl $spo {
            fn boost(&self, v: &SpaceTimeVel) -> Self {
                let g = gamma(v);
                let v = v.r();
                let v2 = v.dot(v).recip();
                let col1 = Vec3::new(g, -g * v.x, -g * v.y);
                let col2 = Vec3::new(
                    -g * v.x,
                    1. + (g - 1.) * v.x * v.x * v2,
                    (g - 1.) * v.x * v.y * v2,
                );
                let col3 = Vec3::new(
                    -g * v.y,
                    (g - 1.) * v.x * v.y * v2,
                    1. + (g - 1.) * v.y * v.y * v2,
                );
                let m = Mat3::from_cols(col1, col2, col3);
                let v = m * self.0;
                Self(v)
            }
        }
    };
}

sp_impl!(SpaceTimePos);
sp_impl!(SpaceTimeVel);

/// Contracts the vector `r` when moving at velocity `u`.
pub(crate) fn l_contract(u: &SpaceTimeVel, r: &SpaceTimePos) -> SpaceTimePos {
    let g = igamma(&u);
    let u = u.r();
    let rt = r.t();
    let r = r.r();
    let rp = r.project_onto(u) * g;
    let rr = r.reject_from(u);
    let ro = rp + rr;
    SpaceTimePos(Vec3::new(rt, ro.x, ro.y))
}

#[derive(Reflect, Component, Default, Deref, Clone, Copy, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct Acc(pub Vec2);

impl Sub<Acc> for Acc {
    type Output = Self;

    fn sub(self, rhs: Acc) -> Self::Output {
        Acc(self.0 - rhs.0)
    }
}

impl Acc {
    pub(crate) fn boost(&self, u: &SpaceTimeVel) -> Self {
        let u2 = u.r();
        let ig = igamma(&u);
        let a = ig * ig * (self.0 - self.0.dot(u2) * u2 * (1. - ig));
        Acc(a)
    }

    pub(crate) fn from_force(m: &Mass, f: &Force) -> Self {
        let a = f.0 * m.0.recip();
        Acc(a)
    }

    pub(crate) fn r(&self) -> Vec2 {
        self.0
    }
}

#[derive(Component, Default)]
pub(crate) struct Epoch(i32);

#[derive(Bundle, Default)]
pub(crate) struct SpaceTimeBundle {
    pub pos: SpaceTimePos,
    pub age: Epoch,
    pub vel: SpaceTimeVel,
    pub acc: Acc,
    pub angle: Angle,
}

pub(crate) fn update_epoch(mut query: Query<(&mut Epoch, &mut SpaceTimePos)>) {
    for (mut epoch, mut pos) in &mut query {
        if pos.t() > 0.9 * f32::MAX {
            epoch.0 += 1;
            pos.0.x = 0.;
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct Mass(pub f32);

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct Force(pub Vec2);

impl std::ops::Deref for Force {
    type Target = Vec2;

    fn deref(&self) -> &Vec2 {
        &self.0
    }
}

#[derive(Component, Default)]
pub(crate) struct Angle(pub f32);

#[derive(Component, Default)]
pub(crate) struct SpaceTimeObject;
