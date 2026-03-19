use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;

use crate::controller::input_handler::*;
use crate::debug::PrototypeMaterial;
use crate::fps::{Floater, FloatMovement};
use crate::utils::{TransformPropagate, TransformPropagateTo};

pub fn spawn_player(commands: &mut Commands, meshes: &mut Assets<Mesh>) -> Entity {
	let camera = commands.spawn((
		Camera3d::default(),
		Transform::from_translation(Vec3::new(0., 0., -0.12)),
		Projection::Perspective(PerspectiveProjection {
			fov: 90.0f32.to_radians(),
			..Default::default()
		}),
	)).id();
	
	let head = commands.spawn((
		Transform::from_translation(Vec3::new(0., 0.5, 0.)),
		TransformPropagate::full().without_rotation(),
	)).add_child(camera).id();

	commands.spawn((
		Name::new("Parker"),
		Collider::capsule(0.28, 0.7),
		Mesh3d(meshes.add(Capsule3d::new(0.28, 0.7))),
		PrototypeMaterial::new("parker"),
		Transform::from_xyz(12., 6., 24.),
		Floater {
			desired_height: 1.45,
			spring_strength: 24.0,
			spring_damp: 0.15,
			..Default::default()
		},
		FloatMovement::default(),
		FpsPlayerInput::default().with_xform_target(camera),
		actions!(FpsPlayerInput[
			(
				Action::<Movement>::new(),
				Bindings::spawn((
					Cardinal::wasd_keys(),
					Axial::left_stick(),
				)),
			),
			(
				Action::<Look>::new(),
				bindings![
					Binding::mouse_motion(),
					(GamepadAxis::RightStickX, Scale::splat(5.0)),
					(GamepadAxis::RightStickY, Scale::splat(5.0), Negate::all(), SwizzleAxis::YXZ),
				],
			),
		]),
	)).add_one_related::<TransformPropagateTo>(head).id()
}
