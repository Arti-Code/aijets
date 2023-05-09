mod ship;
mod util;
mod analizer;
mod resources;

use bevy_inspector_egui::quick::*;
use bevy::{prelude::*, window::WindowResolution, input::common_conditions::input_toggle_active};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ship::*;
pub use crate::resources::*;

pub const WIN_SIZE: Vec2 = Vec2::new(1600.0, 900.0);
pub const SPACE: Vec2 = Vec2::new(3200.0, 1800.0);
pub const SHIPS_NUM: usize = 15;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "iSTAR".to_string(),
                    resolution: WindowResolution::new(WIN_SIZE.x, WIN_SIZE.y),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        ))
        .insert_resource(RapierConfiguration{
            gravity: Vec2::ZERO,
            //gravity: Vect::Y * -0.81 * 10.0,
            timestep_mode: TimestepMode::Fixed { dt: 1.0/30.0, substeps: 1 },
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin {
            enabled: false, 
            always_on_top: true, 
            mode: DebugRenderMode::COLLIDER_SHAPES,
            style: DebugRenderStyle::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.,0.,0.)))
        .add_plugin(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)))
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_graphics_system)
        .add_plugin(ShipPlugin)
        .add_system(camera_control)
        .run();
}

fn setup_graphics_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_control(mut cam_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>, keys: Res<Input<KeyCode>>) {
    for (mut tf, mut projection) in cam_query.iter_mut() {
        if keys.just_pressed(KeyCode::Numpad8) {
            tf.translation.y += 50.0;
        } else if keys.just_pressed(KeyCode::Numpad2) {
            tf.translation.y -= 50.0;
        } else if keys.just_pressed(KeyCode::Numpad4) {
            tf.translation.x -= 50.0;
        } else if keys.just_pressed(KeyCode::Numpad6) {
            tf.translation.x += 50.0;
        } else if keys.just_pressed(KeyCode::Numpad5) {
            tf.translation.x = 0.0;
            tf.translation.y = 0.0;
        } else if keys.just_pressed(KeyCode::NumpadAdd) {
            if projection.scale >= 0.2 {
                projection.scale -= 0.1;
            }
        } else if keys.just_pressed(KeyCode::NumpadSubtract) {
            projection.scale += 0.1;
        } else if keys.just_pressed(KeyCode::NumpadMultiply) {
            projection.scale = 1.0;
        }
    }
}