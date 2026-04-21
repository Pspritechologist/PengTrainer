#![allow(clippy::type_complexity)]

use bevy::{asset::AssetPath, ecs::system::RunSystemOnce, prelude::*};
use avian3d::prelude::*;
use debug::PrototypeMaterial;

mod debug;
mod trenchbroom;
mod movement;
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
		.insert_resource(bevy_egui::EguiGlobalSettings {
			enable_absorb_bevy_input_system: true,
			..Default::default()
		})
		.add_plugins(PhysicsPlugins::default())
		.add_plugins(bevy_enhanced_input::EnhancedInputPlugin)
		.add_plugins(trenchbroom::Plugin)
		.add_plugins((bevy::camera_controller::free_camera::FreeCameraPlugin, bevy_rts_camera::RtsCameraPlugin))
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
		.add_plugins(movement::MovementPlugin)
		.add_plugins(scratch::Plugin);
	
	app.add_systems(Startup, setup);
	
	app.run();
}

fn setup(world: &mut World) -> Result {
	let mut args = std::env::args_os();
	if args.any(|a| a == "--load-map") {
		let Some(map_path) = args.next() else {
			eprintln!("--load-map flag provided but no path given");
			return Err("No path provided to --load-map")?;
		};

		let map_path = AssetPath::from_path_buf(map_path.into()).with_label("Scene");

		let map = SceneRoot(world.resource::<AssetServer>().load(map_path));

		world.spawn((map, Transform::default()));

		world.run_system_once(setup_loaded_map).unwrap();
	} else {
		world.run_system_once(setup_dev_env).unwrap();
	}

	Ok(())
}

fn setup_dev_env(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut scattering_mediums: ResMut<Assets<bevy::pbr::ScatteringMedium>>,
	asset_server: Res<AssetServer>,
) {
	let player = movement::player::spawn_player(&mut commands, scattering_mediums.add(bevy::pbr::ScatteringMedium::default()), &mut meshes)
		.insert(Transform::from_xyz(12., 6., 24.))
		.id();

	commands.spawn((
		Name::new("Cuboid"),
		Collider::cuboid(1.0, 1.0, 1.0),
		Mesh3d(meshes.add(Cuboid::from_length(1.0))),
		PrototypeMaterial::new("cuboid"),
		Transform::from_xyz(0., 20., 0.),
		movement::Floater::default(),
		movement::FloatMovement {
			acceleration: 8.0,
			max_speed: 3.2,
			dimeyness: 4.0,
			..Default::default()
		},
		// scratch::FollowEntity(player),
	));

	commands.spawn((
		Name::new("SwampyPeasants"),
		SceneRoot(asset_server.load("maps/swampypeasants.map#Scene")),
		Transform::from_xyz(0., 0., 0.,),
	)).add_child(player);
}

fn setup_loaded_map(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut scattering_mediums: ResMut<Assets<bevy::pbr::ScatteringMedium>>,
	player_spawn: Option<Single<&GlobalTransform, With<trenchbroom::PlayerSpawn>>>,
) {
	movement::player::spawn_player(&mut commands, scattering_mediums.add(bevy::pbr::ScatteringMedium::default()), &mut meshes)
		.insert(player_spawn.map_or(Transform::from_xyz(0., 5., 0.), |g| g.compute_transform()));
}
