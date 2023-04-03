use crate::physics::{SpaceTimeVel, StationaryFrame, ThreeVector};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{physics::SpaceTimePos, Player};

pub(crate) fn main_ui(
    mut contexts: EguiContexts,
    q_ppos: Query<&SpaceTimePos, With<Player>>,
    q_map: Query<(&SpaceTimePos, &SpaceTimeVel), With<StationaryFrame>>,
) {
    // egui::SidePanel::right("UI")
    let player_pos = q_ppos.single();
    let (map_pos, map_vel) = q_map.single();
    egui::Window::new("Player UI")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            let player_time = format!("Player clock: {:.0}", player_pos.0.t());
            ui.label(player_time);

            let map_time = format!("Galactic clock: {:.0}", map_pos.0.t());
            ui.label(map_time);

            let p = -map_pos.r();
            let player_pos = format!("Player position: {:.1}, {:.1}", p.x, p.y);
            ui.label(player_pos);

            let v = -map_vel.r();
            let player_vel = format!("Player vel: {:.2}, {:.2}", v.x, v.y);
            ui.label(player_vel);
        });
}
