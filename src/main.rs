#![allow(clippy::type_complexity)]

use bevy::{camera_controller::free_camera::FreeCameraPlugin, prelude::*};
use avian3d::prelude::*;
use bevy_rts_camera::RtsCameraPlugin;
use debug::PrototypeMaterial;

mod debug;
mod trenchbroom;
mod controller;
mod rts;
mod fps;
mod utils;

mod scratch;

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
		.add_plugins(utils::UtilsPlugin)
		.add_plugins(controller::Plugin)
		.add_plugins(fps::FpsPlayerPlugin)
		.add_plugins(scratch::Plugin)
		.add_systems(PostStartup, setup);

	app.run();
}

fn setup(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	asset_server: Res<AssetServer>,
) {
	commands.spawn((
		Name::new("SwampyPeasants"),
		SceneRoot(asset_server.load("maps/swampypeasants.map#Scene")),
		Transform::from_xyz(0., 0., 0.,),
		RigidBody::Static,
	));

	let player = fps::player::spawn_player(&mut commands, &mut meshes);

	commands.spawn((
		Name::new("Cuboid"),
		Collider::cuboid(1.0, 1.0, 1.0),
		Mesh3d(meshes.add(Cuboid::from_length(1.0))),
		PrototypeMaterial::new("cuboid"),
		Transform::from_xyz(0., 20., 0.),
		fps::Floater::default(),
		fps::FloatMovement::default(),
		// FollowEntity(player),
	));

	// commands.spawn((
	// 	Camera3d::default(),
	// 	Camera {
	// 		is_active: false,
	// 		..Default::default()
	// 	},
	// 	Transform::from_xyz(-7., 4.5, 20.0).looking_at(Vec3::new(16., 4.5, 30.), Vec3::Y),
	// 	FreeCamera {
	// 		sensitivity: 0.2,
	// 		friction: 25.0,
	// 		walk_speed: 3.0,
	// 		run_speed: 9.0,
	// 		..default()
	// 	},
	// ));
}
