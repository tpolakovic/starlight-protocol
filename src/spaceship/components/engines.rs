use bevy::prelude::*;

// use crate::physics::{Force, Mass};

#[derive(Component)]
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

#[derive(Component)]
pub(crate) struct Brake {
    thrust: f32,
    running: bool,
}

impl Default for Brake {
    fn default() -> Self {
        Brake {
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

#[derive(Bundle)]
pub(crate) struct EngineBrakeBundle {
    engine: Engine,
    brake: Brake,
    vector: VectorEngine,
}

// pub(crate) fn linear_engine_system(mut e_query: Query<(&Engine, &Brake, &Angle, &mut Force)>) {
//     for (e, b, a, mut f) in &mut e_query
//         .iter_mut()
//         .filter(|(e, b, _, _)| e.running | b.running)
//     {
//         let thrust = (e.running as i32 as f32) * e.thrust - (b.running as i32 as f32) * b.thrust;
//         f.0 = Vec2::from_angle(a.0) * thrust;
//     }
// }

// pub(crate) fn vector_engine_system(
//     mut v_query: Query<(&VectorEngine, &mut Angle, &Mass)>,
//     dt: Res<FixedTime>,
// ) {
//     for (ve, mut a, m) in &mut v_query {
//         match ve.mode {
//             VectorEngineMode::Right => {
//                 a.0 += ve.angular_v * dt.period.as_secs_f32() * m.0.recip();
//             }
//             VectorEngineMode::Left => {
//                 a.0 -= ve.angular_v * dt.period.as_secs_f32() * m.0.recip();
//             }
//             VectorEngineMode::Off => (),
//         }
//     }
// }
