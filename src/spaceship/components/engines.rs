use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::physics::{Force, RealGlobalTransform};

// use crate::physics::{Force, Mass};

#[derive(Reflect, Component, InspectorOptions)]
pub(crate) struct Engine {
    thrust: f32,
    running: bool,
}

impl Default for Engine {
    fn default() -> Self {
        Engine {
            thrust: 0.,
            running: false,
        }
    }
}

pub(crate) enum VectorEngineMode {
    Right,
    Left,
    Off,
}

#[derive(Component)]
pub(crate) struct VectorEngine {
    mode: VectorEngineMode,
    angular_v: f32,
}

impl Default for VectorEngine {
    fn default() -> Self {
        VectorEngine {
            mode: VectorEngineMode::Off,
            angular_v: 0.,
        }
    }
}

pub(crate) fn linear_engine_system(mut query: Query<(&Engine, &RealGlobalTransform, &mut Force)>) {
    for (engine, RealGlobalTransform(global_transform), mut force) in &mut query.iter_mut() {
        if engine.running {
            let thrust = engine.thrust;
            let (_, angle_quaternion, _) = global_transform.to_scale_rotation_translation();
            let (_, angle) = angle_quaternion.to_axis_angle();
            let engine_force = Vec2::from_angle(angle) * thrust;
            force.0 += engine_force;
        }
    }
}
