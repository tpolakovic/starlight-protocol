use bevy::prelude::*;

mod spacetime;
pub(crate) use spacetime::*;

mod frame_transforms;
pub(crate) use frame_transforms::*;

mod dynamics;
pub(crate) use dynamics::*;

mod objects;
pub(crate) use objects::*;

// Unit Converters

// Base distance unit is ~1/1000 distance to Proxima Centauri.

/// Convert world units to meters.
pub(crate) fn to_meters(x: f32) -> f32 {
    x * 4.018E-13
}

/// Convert world units to meters.
pub(crate) fn from_meters(x: f32) -> f32 {
    x * 4.018E13
}

// World speed unit is c.

/// Convert speed in km/s to fraction of c.
pub(crate) fn to_c(x: f32) -> f32 {
    x / 186000.
}

/// Convert speed in fraction of c to km/s.
pub(crate) fn from_c(x: f32) -> f32 {
    x * 186000.
}

// Base mass unit is a megaton.

/// Converts mass in kg to megaton.
pub(crate) fn to_mt(x: f32) -> f32 {
    x / 1000000.
}

/// Converts mass from megaton to kg.
pub(crate) fn from_mt(x: f32) -> f32 {
    x * 1000000.
}

#[derive(Resource)]
pub(crate) struct LengthFactor(pub f32);

impl Default for LengthFactor {
    fn default() -> Self {
        LengthFactor(1.)
    }
}

#[derive(Resource)]
pub(crate) struct TimeFactor(pub f32);

impl Default for TimeFactor {
    fn default() -> Self {
        TimeFactor(30.)
    }
}

pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LengthFactor(1.))
            .insert_resource(TimeFactor(1.))
            .add_system(redraw_in_player_frame)
            .add_system(update_epoch)
            .add_systems(
                (
                    update_acc,
                    update_vel.after(update_acc),
                    update_pos.after(update_vel),
                )
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
    }
}
