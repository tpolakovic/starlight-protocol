use bevy::prelude::*;

mod components;

struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems((engines::linear_engine_system, engines::vector_engine_system));
    }
}
