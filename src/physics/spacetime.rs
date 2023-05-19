use std::ops::Sub;

use bevy::{math::Vec3A, prelude::*};
use bevy_inspector_egui::prelude::*;
use big_space::GridCell;

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

#[derive(Reflect, Default, Component, InspectorOptions)]
#[reflect(Component)]
/// Time in the local frame.
///
/// This is the time that passes for an observer at rest in the local frame, or, it's the zeroth component of the three-position.
pub(crate) struct LocalTime(pub f64);

#[derive(Reflect, Default, Deref, Clone, Copy, Component, InspectorOptions)]
#[reflect(Component)]
/// Two-velocity of an object in spacetime.
///
/// This is the space-like component of the three-velocity of an object.
pub(crate) struct Velocity(pub Vec2);

pub(crate) trait Contract {
    fn contract(&self, v: &Vec2) -> Self;
}

pub(crate) trait Boost {
    fn boost(&self, v: &Vec2) -> Self;
}

impl Contract for Vec3A {
    fn contract(&self, v: &Vec2) -> Self {
        let u = Vec3A::new(v.x, v.y, self.z);
        let g = igamma(v);
        let p = self.project_onto(u) * g;
        let r = self.reject_from(u);
        p + r
    }
}

#[derive(Component, Default, Deref, Clone, Copy)]
/// Inverse mass of an object.
///
/// Value of `0.` means infinite mass.
pub(crate) struct InverseMass(pub f32);

impl InverseMass {
    pub fn from_mass(mass: f32) -> Self {
        if mass == 0. {
            return InverseMass(0.);
        }
        InverseMass(mass.recip())
    }

    pub fn to_mass(self) -> f32 {
        if self.0 == 0. {
            return 0.;
        }
        self.0.recip()
    }
}

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
/// Two-force applied to an object in spacetime.
pub(crate) struct Force(pub Vec2);

impl Force {
    pub const ZERO: Self = Force(Vec2::ZERO);
}

#[derive(Reflect, Component, Default, Deref, Clone, Copy, InspectorOptions)]
#[reflect(Component)]
/// Two-acceleration of an object in spacetime.
pub(crate) struct Acceleration(pub Vec2);

impl Sub<Acceleration> for Acceleration {
    type Output = Self;

    fn sub(self, rhs: Acceleration) -> Self::Output {
        Acceleration(self.0 - rhs.0)
    }
}

impl Acceleration {
    /// Returns the acceleration of an object with mass `m` and force `f`.
    ///
    /// Entities with zero mass will always have zero proper acceleration.
    pub(crate) fn from_force(InverseMass(inverse_mass): &InverseMass, Force(f): &Force) -> Self {
        let a = *f * *inverse_mass;
        Acceleration(a)
    }
}

impl Boost for Acceleration {
    fn boost(&self, v: &Vec2) -> Self {
        let ig = igamma(v);
        let a = ig * ig * (self.0 - *v * self.0.dot(*v) * (1. - ig));
        Acceleration(a)
    }
}

#[derive(Component, Default)]
/// Marker struct used to denote that an entity is an object to be simulated in the spacetime.
pub(crate) struct SpaceTimeObject;

#[derive(Reflect, Component, Default, InspectorOptions)]
pub(crate) struct RealGlobalTransform(pub GlobalTransform);

#[derive(Bundle, Default)]
pub(crate) struct SpaceTimeBundle {
    pub time: LocalTime,
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub force: Force,
    pub inverse_mass: InverseMass,
    pub real_global_transform: RealGlobalTransform,
    pub spo: SpaceTimeObject,
    pub gc: GridCell<i64>,
}
