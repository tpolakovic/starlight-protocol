use bevy::prelude::*;

mod spacetime;
use big_space::propagation::propagate_transforms;
pub(crate) use spacetime::*;

mod frame_transforms;
pub(crate) use frame_transforms::*;

mod dynamics;
pub(crate) use dynamics::*;

mod objects;
pub(crate) use objects::*;

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
                    update_acc,
                    update_vel.after(update_acc),
                    update_pos.after(update_vel),
                )
                    .in_schedule(CoreSchedule::FixedUpdate),
            );
    }
}
