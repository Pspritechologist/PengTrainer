use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;

use crate::controller::input_handler::*;
use crate::debug::PrototypeMaterial;
use crate::fps::{Floater, FloatMovement};
use crate::utils::{TransformPropagateFrom, TransformPropagateTo};

pub fn player_bundle(meshes: &mut Assets<Mesh>) -> impl Bundle {
	(
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
		FpsPlayerInput::default(),
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
		related!(TransformPropagateFrom[
			(
				Camera3d::default(),
				Transform::from_translation(Vec3::new(0., 0.5, -0.12)),
				Projection::Perspective(PerspectiveProjection {
					fov: 90.0f32.to_radians(),
					..Default::default()
				}),
				TransformPropagateTo::full().without_rotation(),
			),
		]),
	)
}
