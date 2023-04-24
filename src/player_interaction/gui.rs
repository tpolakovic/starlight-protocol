use std::ops::Neg;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::physics::{LocalTime, Player, StationaryFrame, Velocity};

pub(crate) fn main_ui(
    mut contexts: EguiContexts,
    q_ppos: Query<&LocalTime, With<Player>>,
    q_map: Query<(&Transform, &LocalTime, &Velocity), With<StationaryFrame>>,
) {
    let LocalTime(player_t) = q_ppos.single();
    let (map_pos, LocalTime(map_t), Velocity(map_vel)) = q_map.single();
    egui::Window::new("Player UI")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            let player_time = format!("Player clock: {:.0}", player_t);
            ui.label(player_time);

            let map_time = format!("Galactic clock: {:.0}", map_t);
            ui.label(map_time);

            let p = map_pos.translation.neg();
            let player_pos = format!("Player position: {:.1}, {:.1}", p.x, p.y);
            ui.label(player_pos);

            let v = map_vel.neg();
            let player_vel = format!("Player vel: |{:.2}, {:.2}| = {:.2}", v.x, v.y, v.length());
            ui.label(player_vel);
        });
}
