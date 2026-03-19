use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use tracing::instrument;

use super::MovementInput;

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(EnhancedInputPlugin)
			.add_input_context::<FpsPlayerInput>()
			.add_input_context::<RtsPlayerInput>()
			.add_systems(FixedPostUpdate, FpsPlayerInput::post_update)
			.add_observer(FpsPlayerInput::on_move_input)
			.add_observer(FpsPlayerInput::on_look_input)
			;
	}
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct FpsPlayerInput {
	look_sensitivity: f32,
}
impl Default for FpsPlayerInput {
	fn default() -> Self {
		Self { look_sensitivity: 0.01 }
	}
}

impl FpsPlayerInput {
	#[instrument(skip_all)]
	fn on_move_input(
		move_input: On<Fire<Movement>>,
		query: Query<(
			Option<&Transform>,
			&mut MovementInput,
		), With<FpsPlayerInput>>,
	) {
		let Ok((xform, mut target)) = query.get_inner(move_input.context) else {
			return;
		};
		
		debug!("Move input: {}", move_input.value);

		// -z Forwards, +z Backwards, -x Left, +x Right
		let mut movement = move_input.value.xxy() * Vec3::new(1., 0., -1.);

		if let Some(xform) = xform {
			movement = xform.rotation.mul_vec3(movement);
		}

		target.movement = movement; //? Gets overwritten.
	}

	#[instrument(skip_all)]
	fn on_look_input(
		look_input: On<Fire<Look>>,
		query: Query<(
			&FpsPlayerInput,
			Option<&Transform>,
			&mut MovementInput,
		)>,
	) {
		let Ok((player_input, xform, mut target)) = query.get_inner(look_input.context) else {
			return;
		};

		debug!("Look input: {}", look_input.value);
		let mut look = look_input.value.yxx().with_z(0.) * player_input.look_sensitivity;

		if let Some(xform) = xform {
			look = xform.rotation.mul_vec3(look);
		}

		target.look += look; //? Gets accumulated.
	}

	fn post_update(
		query: Query<&mut MovementInput, (With<FpsPlayerInput>, Changed<MovementInput>)>,
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
