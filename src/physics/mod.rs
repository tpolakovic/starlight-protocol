use bevy::prelude::*;
use big_space::propagation::propagate_transforms;

pub(crate) mod spacetime;

pub(crate) mod frame_transforms;

pub(crate) mod dynamics;

pub(crate) mod objects;

use crate::spaceship::components::engines::linear_engine_system;

use self::{
    dynamics::{clear_forces, update_acceleration, update_position, update_velocity},
    frame_transforms::redraw_in_player_frame,
};

#[derive(Resource)]
pub(crate) struct TimeFactor(pub f32);

impl Default for TimeFactor {
    fn default() -> Self {
        TimeFactor(1.)
    }
}

pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeFactor::default())
            .add_systems((redraw_in_player_frame
                .after(propagate_transforms::<i64>)
                .in_base_set(CoreSet::PostUpdate),))
            .add_systems(
                (
                    linear_engine_system,
                    update_acceleration,
                    update_velocity,
                    update_position,
                    clear_forces,
                )
                    .chain()
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
    }
}
