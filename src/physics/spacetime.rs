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

#[inline]
/// Inverse of Lorentz factor.
pub(crate) fn igamma(v: &Vec2) -> f32 {
    f32::sqrt(1. - v.length_squared())
}

#[inline]
/// Lorentz factor.
pub(crate) fn gamma(v: &Vec2) -> f32 {
    igamma(v).recip()
}

/// Contracts the vector `v` when moving at velocity `u`.
pub(crate) fn l_contract(u: &Vec3, v: &Vec2) -> Vec2 {
    let u = u.r();
    let g = igamma(&u);
    let vp = v.project_onto(u) * g;
    let vr = v.reject_from(u);
    vp + vr
}

macro_rules! sp_deref_impl {
    ($spo:ty) => {
        impl std::ops::Deref for $spo {
            type Target = Vec3;

            fn deref(&self) -> &Vec3 {
                &self.0
            }
        }
    };
}

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct SpaceTimePos(pub Vec3);
sp_deref_impl!(SpaceTimePos);

#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct SpaceTimeVel(pub Vec3);
sp_deref_impl!(SpaceTimeVel);

impl Default for SpaceTimeVel {
    fn default() -> Self {
        SpaceTimeVel(Vec3::new(1., 0., 0.))
    }
}

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct SpaceTimeAcc(pub Vec2);

impl SpaceTimeAcc {
    pub(crate) fn boost(&self, u: &SpaceTimeVel) -> Self {
        let u2 = u.r();
        let ig = igamma(&u2);
        let a = ig * ig * (self.0 - self.0.dot(u2) * u2 * (1. - ig));
        SpaceTimeAcc(a)
    }

    #[inline]
    pub(crate) fn from_force(m: &Mass, f: &Force) -> Self {
        let a = f.0 * m.0.recip();
        SpaceTimeAcc(a)
    }
}

#[derive(Bundle, Default)]
pub(crate) struct SpaceTimeBundle {
    pub pos: SpaceTimePos,
    pub vel: SpaceTimeVel,
    pub acc: SpaceTimeAcc,
    pub angle: Angle,
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
