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
pub(crate) struct LocalTime(pub f32);

#[derive(Reflect, Default, Component, InspectorOptions, Deref)]
#[reflect(Component)]
pub(crate) struct Vel(pub Vec2);

pub(crate) trait Contract {
    fn contract(&self, v: &Vec2) -> Self;
}

impl Contract for Vec3A {
    fn contract(&self, v: &Vec2) -> Self {
        let u = Vec3A::new(v.x, v.y, 0.);
        let g = igamma(v);
        let p = self.project_onto(u) * g;
        let r = self.reject_from(u);
        p + r
    }
}

/// Contracts the vector `r` when moving at velocity `u`.
pub(crate) fn l_contract(u: &Vec2, r: &Vec2) -> Vec2 {
    let g = igamma(u);
    let rp = r.project_onto(*u) * g;
    let rr = r.reject_from(*u);
    let ro = rp + rr;
    Vec2::new(ro.x, ro.y)
}

#[derive(Component, Default)]
pub(crate) struct Mass(pub f32);

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct Force(pub Vec2);

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
    pub(crate) fn boost(&self, u: &Vec2) -> Self {
        let ig = igamma(u);
        let a = ig * ig * (self.0 - *u * self.0.dot(*u) * (1. - ig));
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

pub(crate) fn update_epoch(mut query: Query<(&mut Epoch, &mut LocalTime)>) {
    for (mut epoch, mut t) in &mut query {
        if t.0 > 0.9 * f32::MAX {
            epoch.0 += 1;
            t.0 = 0.;
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct SpaceTimeObject;

#[derive(Bundle, Default)]
pub(crate) struct SpaceTimeBundle {
    pub time: LocalTime,
    pub age: Epoch,
    pub vel: Vel,
    pub acc: Acc,
    spo: SpaceTimeObject,
    gc: GridCell<i64>,
}
