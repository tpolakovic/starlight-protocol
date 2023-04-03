#![allow(clippy::type_complexity)]
#![allow(dead_code)]

mod physics;
use physics::*;
mod player_interaction;
use player_interaction::*;
mod spaceship;
use spaceship::*;

use std::f32::consts::PI;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use rand::prelude::*;

const TIME_STEP: f32 = 1. / 60.;

const BACKGROUND_COLOR: Color = Color::hsl(221., 0.17, 0.22);

const WORLD_SIZE: [f32; 2] = [2000., 1000.];
const GRID_N: [i32; 2] = [200, 100];

const N_DEBRIS: u32 = 100;

pub(crate) fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Starlight Protocol".into(),
                ..default()
            }),
            ..default()
        }))
        // .add_system(setup2.in_schedule(CoreSchedule::Startup))
        // .add_system(sprite_movement.in_base_set(CoreSet::Update))
        // .add_system(
        //     test.in_base_set(CoreSet::PostUpdate)
        //         .after(TransformSystem::TransformPropagate),
        // )
        .add_plugin(EguiPlugin)
        .register_type::<Force>()
        .register_type::<SpaceTimePos>()
        .insert_resource(FixedTime::new_from_secs(TIME_STEP))
        .add_startup_system(setup)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_system(main_ui)
        .add_startup_system(spawn_debris)
        .add_startup_system(spawn_player)
        .add_plugin(PhysicsPlugin)
        .add_plugin(FilterQueryInspectorPlugin::<With<Player>>::default())
        .add_system(bevy::window::close_on_esc)
        .run();
}

// #[derive(Component)]
// enum Direction {
//     Up,
//     Down,
// }

// fn setup2(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn((
//         SpriteBundle {
//             texture: asset_server.load("icon.png"),
//             transform: Transform::from_xyz(100., 0., 0.),
//             ..default()
//         },
//         Direction::Up,
//     ));
// }

// fn test(mut q: Query<&mut GlobalTransform, With<Direction>>) {
//     let mut gt = q.single_mut();
//     let a = gt.affine();
//     let m1 = a.matrix3;
//     let m2 = Mat3A::from_scale(Vec2::new(2.0, 1.0));
//     let a2 = Affine3A {
//         matrix3: m1 * m2,
//         translation: a.translation + Vec3A::new(-300., 0., 0.),
//     };
//     // let a3 = a * a2;
//     *gt = a2.into();
// }

// /// The sprite is animated by changing its translation depending on the time that has passed since
// /// the last frame.
// fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
//     for (mut logo, mut transform) in &mut sprite_position {
//         match *logo {
//             Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
//             Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
//         }

//         if transform.translation.y > 200. {
//             *logo = Direction::Down;
//         } else if transform.translation.y < -200. {
//             *logo = Direction::Up;
//         }
//     }
// }

// Resources

// Components
#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
struct DisplayName(String);

// Systems
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let xn = GRID_N[0] / 2;
    let yn = GRID_N[1] / 2;
    let xs = WORLD_SIZE[0] / xn as f32;
    let ys = WORLD_SIZE[1] / yn as f32;
    let meshhandle: Mesh2dHandle = meshes.add(shape::Circle::new(1.).into()).into();
    let materialhandle = materials.add(ColorMaterial::from(Color::hex("80a0c2").unwrap()));
    for i in -xn..xn {
        for j in -yn..yn {
            let pos = Vec3::new(0., i as f32 * xs, j as f32 * ys);

            let spobject = SpaceTimeBundle {
                pos: SpaceTimePos(pos),
                ..default()
            };
            let bundle = (
                MaterialMesh2dBundle {
                    mesh: meshhandle.clone(),
                    material: materialhandle.clone(),
                    ..default()
                },
                Force::default(),
            );
            spawn_as_spacetime_object!(commands, spobject, bundle, 1.);
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
            0.,
            (rng.gen::<f32>() - 0.5) * 2. * WORLD_SIZE[0],
            (rng.gen::<f32>() - 0.5) * 2. * WORLD_SIZE[1],
        );
        let a = rng.gen::<f32>() * PI;
        let texture_handle = asset_server.load("textures/debris.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(100., 100.), 5, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let spobject = SpaceTimeBundle {
            pos: SpaceTimePos(pos),
            angle: Angle(a),
            ..default()
        };
        let bundle = (
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(rng.gen_range(1..5)),
                transform: Transform::from_rotation(Quat::from_rotation_z(a)),
                ..default()
            },
            Mass(100.),
            Force::default(),
        );
        spawn_as_spacetime_object!(commands, spobject, bundle, 5.);
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
    ));
    commands.spawn((
        SpaceTimeBundle::default(),
        Mass(1.),
        Force::default(),
        TransformBundle::default(),
        StationaryFrame,
    ));
}
