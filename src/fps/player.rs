use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;

use crate::controller::input_handler::*;
use crate::debug::PrototypeMaterial;
use crate::fps::{Floater, FloatMovement};

pub fn player_bundle(meshes: &mut Assets<Mesh>) -> impl Bundle {
	(
		Name::new("Parker"),
		Collider::capsule(0.28, 0.7),
		Mesh3d(meshes.add(Capsule3d::new(0.28, 0.7))),
		PrototypeMaterial::new("parker"),
		Transform::from_xyz(12., 6., 24.),
		Floater {
			desired_height: 1.45,
			// spring_strength: 0.0,
			..Default::default()
		},
		FloatMovement::default(),
		crate::controller::MovementInput::default(),
		actions!(FpsPlayerInput[
			(
				Action::<Movement>::new(),
				DeadZone::default(),
				// SmoothNudge::default(),
				// DeltaScale::default(),
				// Scale::splat(30.),
				Bindings::spawn((
					Cardinal::wasd_keys(),
					Axial::left_stick(),
				)),
			),
			(
				Action::<Look>::new(),
				DeadZone::default(),
				SmoothNudge::default(),
				// DeltaScale::default(),
				// Scale::splat(30.),
				Bindings::spawn((
					Spawn(Binding::mouse_motion()),
					Axial::right_stick(),
				)),
			),
		]),
		children![
			(
				Camera3d::default(),
				Projection::Perspective(PerspectiveProjection {
					fov: 90.0f32.to_radians(),
					..Default::default()
				}),
			),
		]
	)
}
