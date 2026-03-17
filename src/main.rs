#![allow(clippy::type_complexity)]

use bevy::{camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, prelude::*};
use avian3d::prelude::*;
use bevy_rts_camera::RtsCameraPlugin;
use debug::PrototypeMaterial;

mod debug;
mod trenchbroom;
mod controller;
mod rts;
mod fps;

fn main() {
	let mut app = App::new();

	app
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				present_mode: bevy::window::PresentMode::AutoNoVsync,
				resolution: (1920, 1080).into(),
				mode: bevy::window::WindowMode::Windowed,
				..Default::default()
			}),
			..Default::default()
		}))
		.add_plugins(PhysicsPlugins::default())
		.add_plugins(trenchbroom::Plugin)
		.add_plugins((FreeCameraPlugin, RtsCameraPlugin))
		.add_plugins(debug::PrototypeMaterialPlugin)
		.add_plugins(debug::InspectorPlugin);

	if std::env::args_os().any(|a| a == "--phys-debug") {
		app.add_plugins(PhysicsDebugPlugin);
	}

	if std::env::args_os().any(|a| a == "--fps") {
		app.add_plugins(debug::FpsOverlay);
	}

	app
		.add_plugins(controller::Plugin)
		.add_plugins(fps::FpsPlayerPlugin)
		.add_systems(FixedUpdate, throw_balls)
		.add_systems(PostStartup, setup);

	app.run();
}

fn throw_balls(
	entities: Query<&mut LinearVelocity, With<trenchbroom::Ball>>,
) {
	for mut vel in entities {
		if fastrand::f32() < 0.002 {
			vel.0 += Vec3::new(
				fastrand::f32() * 30.0 - 15.0,
				fastrand::f32() * 10.0 - 6.5,
				fastrand::f32() * 30.0 - 15.0,
			);
		}
	}
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		SceneRoot(asset_server.load("maps/swampypeasants.map#Scene")),
		Transform::from_xyz(0., 0., 0.,),
		RigidBody::Static,
	));

	commands.spawn((
		Collider::cuboid(1.0, 1.0, 1.0),
		Mesh3d(meshes.add(Cuboid::from_length(1.0))),
		PrototypeMaterial::new("cuboid"),
		Transform::from_xyz(0., 20., 0.),
		fps::Floater::default(),
	));

	commands.spawn(fps::player::player_bundle(&mut meshes));

	// commands.spawn((
	// 	Camera3d::default(),
	// 	Projection::Orthographic(OrthographicProjection {
    //         scale: 0.032,
    //         near: 0.0,
    //         far: 1000.0,
	// 		..OrthographicProjection::default_3d()
	// 	}),
	// 	// Transform::from_xyz(-5.5, 4.5, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
	// 	RtsCamera::default(),
	// 	// RtsCameraControls::default(),
	// 	RtsCameraControls {
	// 		key_up: KeyCode::KeyW,
	// 		key_down: KeyCode::KeyS,
	// 		key_left: KeyCode::KeyA,
	// 		key_right: KeyCode::KeyD,
	// 		lock_on_rotate: true,
	// 		lock_on_drag: true,
	// 		button_drag: Some(MouseButton::Left),
	// 		button_rotate: MouseButton::Right,
	// 		zoom_sensitivity: 0.2,
	// 		..Default::default()
	// 	},
	// 	Mesh3d(meshes.add(Sphere::new(0.5))),
	// 	PrototypeMaterial::new("camera"),
	// ));

	commands.spawn((
		Camera3d::default(),
		Camera {
			is_active: false,
			..Default::default()
		},
		Transform::from_xyz(-7., 4.5, 20.0).looking_at(Vec3::new(16., 4.5, 30.), Vec3::Y),
		FreeCamera {
			sensitivity: 0.2,
			friction: 25.0,
			walk_speed: 3.0,
			run_speed: 9.0,
			..default()
		},
	));
}
