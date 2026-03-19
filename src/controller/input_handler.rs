use bevy::{input::InputSystems, prelude::*};
use bevy_enhanced_input::prelude::*;
use tracing::instrument;

use super::MovementInput;

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnhancedInputPlugin)
			.add_input_context::<FpsPlayerInput>()
			.add_input_context::<RtsPlayerInput>()
			.add_systems(PreUpdate, FpsPlayerInput::update.after(InputSystems))
			.add_systems(FixedPostUpdate, FpsPlayerInput::post_update);
	}
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
#[require(Name::new("FpsPlayerInput"))]
pub struct FpsPlayerInput;

impl FpsPlayerInput {
	#[instrument(skip_all)]
	fn update(
		query: Query<(
			&Actions<FpsPlayerInput>,
			Option<&Transform>,
			&mut MovementInput,
		)>,
		move_inputs: Query<&Action<Movement>>,
		look_inputs: Query<&Action<Look>>,
	) {
		for (actions, xform, mut target) in query {
			// -z Forwards, +z Backwards, -x Left, +x Right
			let mut movement = move_inputs.iter_many(actions)
				.next()
				.map(|move_input| move_input.yxx().with_y(0.))
				.unwrap_or_default();

			if let Some(xform) = xform {
				movement = xform.rotation.mul_vec3(movement);
			}

			// let look = look_input.yxx().with_z(0.);
			let look = look_inputs.iter_many(actions)
				.next()
				.map(|look_input| look_input.yxx().with_z(0.) * 0.002)
				.unwrap_or_default();

			target.movement = movement; //? Gets overwritten.
			target.look += look; //? Gets accumulated.
		}
	}

	fn post_update(
		query: Query<&mut MovementInput, With<FpsPlayerInput>>,
	) {
		for mut target in query {
			target.movement = Vec3::ZERO;
			target.look = Vec3::ZERO;
		}
	}
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct RtsPlayerInput;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Movement;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Look;
