use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_pretty_nice_input::prelude::*;

use crate::debug::PrototypeMaterial;
use crate::fps::{Floater, FloatMovement};
use crate::controller::input_handler::PlayerInput;

pub fn player_bundle(meshes: &mut Assets<Mesh>) -> impl Bundle {
	(
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
		PlayerInput::default(),
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
