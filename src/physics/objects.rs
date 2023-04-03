use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
// use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub(crate) struct Player;

#[derive(Component, Default)]
pub(crate) struct StationaryFrame;

#[derive(Component, Default)]
pub(crate) struct Star;
