#![allow(clippy::type_complexity)]
#![allow(dead_code)]

mod physics;
use physics::*;
mod player_interaction;
use player_interaction::*;
mod spaceship;

use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use big_space::{FloatingOrigin, FloatingOriginPlugin, GridCell};
use rand::prelude::*;

const TIME_STEP: f32 = 1. / 60.;

const BACKGROUND_COLOR: Color = Color::hsl(221., 0.17, 0.22);

const WORLD_SIZE: [f32; 2] = [2000., 1000.];
const GRID_N: [i32; 2] = [20, 10];

const N_DEBRIS: u32 = 100;

pub(crate) fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Starlight Protocol".into(),
                        ..default()
                    }),
                    ..default()
                })
                .build()
                .disable::<TransformPlugin>(),
        )
        .add_plugin(FloatingOriginPlugin::<i64>::default())
        .add_plugin(EguiPlugin)
        .register_type::<Force>()
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_startup_system(setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(main_ui)
        .add_startup_system(spawn_grid)
        .add_startup_system(spawn_player)
        .add_plugin(PhysicsPlugin)
        .add_plugin(FilterQueryInspectorPlugin::<With<Player>>::default())
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
struct DisplayName(String);

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        GridCell::<i64>::default(),
        FloatingOrigin,
    ));
}

#[derive(Component, Default)]
struct GridPoint;

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_handle: bevy::sprite::Mesh2dHandle = meshes.add(Mesh::from(shape::Quad::default())).into();
    let material = materials.add(ColorMaterial::from(Color::WHITE));
    for i in 0..GRID_N[0] {
        for j in 0..GRID_N[1] {
            let x = (i - GRID_N[0]/2) as f32 * (WORLD_SIZE[0] / GRID_N[0] as f32);
            let y = (j - GRID_N[1]/2) as f32 * (WORLD_SIZE[1] / GRID_N[1] as f32);
            let t = Transform::from_xyz(x, y, 0.).with_scale(Vec3::splat(9.));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone(),
                    transform: t,
                    material: material.clone(),
                    ..default()
                },
                Mass(1.),
                Force::default(),
                SpaceTimeBundle::default(),
                GridPoint,
            ));
        }
    }
}

fn spawn_debris(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..N_DEBRIS {
        let pos = Vec3::new(
            (rng.gen::<f32>() - 0.5) * 2. * WORLD_SIZE[0],
            (rng.gen::<f32>() - 0.5) * 2. * WORLD_SIZE[1],
            1.,
        );
        let a = rng.gen::<f32>() * PI;
        let texture_handle = asset_server.load("textures/debris.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(100., 100.), 5, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(rng.gen_range(1..5)),
                transform: Transform::from_rotation(Quat::from_rotation_z(a)).with_translation(pos),
                ..default()
            },
            Mass(100.),
            Force::default(),
            SpaceTimeBundle::default(),
            //SpaceTimeObject,
            //GridCell::<i64>::default(),
        ));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/space_ship_player.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(50., 50.), 2, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(Vec3::new(0., 0., 10.))
                .with_scale(Vec3::new(1., 1., 0.))
                .with_rotation(Quat::from_rotation_z(0.)),
            ..default()
        },
        Mass(2.),
        Force::default(),
        SpaceTimeBundle::default(),
        Player,
        //GridCell::<i64>::default(),
    ));
    commands.spawn((
        Mass(1.),
        Force::default(),
        TransformBundle::default(),
        StationaryFrame,
        SpaceTimeBundle::default(),
        //GridCell::<i64>::default(),
    ));
}
