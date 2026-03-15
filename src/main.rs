use avian3d::{PhysicsPlugins, prelude::{AngularVelocity, Collider, PhysicsDebugPlugin, RigidBody}};
use bevy::{camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, prelude::*};
use bevy_rts_camera::RtsCameraPlugin;
use debug::PrototypeMaterial;

mod debug;
mod trenchbroom;
mod rts;

fn main() {
	let mut app = App::new();

	app.add_plugins((DefaultPlugins, PhysicsPlugins::default()))
		.add_plugins(trenchbroom::Plugin)
		.add_plugins((FreeCameraPlugin, RtsCameraPlugin))
        // .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
		.add_plugins(debug::PrototypeMaterialPlugin)
		.add_systems(PostStartup, setup);

	if std::env::args_os().any(|a| a.eq_ignore_ascii_case("--phys-debug")) {
		app.add_plugins(PhysicsDebugPlugin);
	}

	if std::env::args_os().any(|a| a.eq_ignore_ascii_case("--fps")) {
		app.add_plugins(debug::fps_overlay());
	}

	app.run();
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
		RigidBody::Dynamic,
		Collider::cuboid(1.0, 1.0, 1.0),
		AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
		Mesh3d(meshes.add(Cuboid::from_length(1.0))),
		PrototypeMaterial::new("cuboid"),
		Transform::from_xyz(0.0, 20.0, 0.0),
	));

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
		Transform::from_xyz(-7., 4.5, 20.0).looking_at(Vec3::new(-5.5, 4.5, 18.0), Vec3::Y),
		FreeCamera {
			sensitivity: 0.2,
			friction: 25.0,
			walk_speed: 3.0,
			run_speed: 9.0,
			..default()
		},
	));
}
