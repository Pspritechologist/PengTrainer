use bevy::{input::{InputSystems, mouse::AccumulatedMouseMotion}, prelude::*};
use tracing::instrument;

use super::MovementInput;

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(PreUpdate, PlayerInput::update.after(InputSystems))
			.add_systems(FixedPostUpdate, PlayerInput::post_update);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum InputSource {
	#[default]
	KeyboardMouse,
	Gamepad(usize),
	Disabled,
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component)]
#[require(MovementInput)]
pub struct PlayerInput {
	source: InputSource,
	look_sensitivity: f32,
}
impl Default for PlayerInput {
	fn default() -> Self {
		Self {
			source: InputSource::default(),
			look_sensitivity: 0.001,
		}
	}
}
impl PlayerInput {
	#[instrument(skip_all)]
	fn update(
		key_input: Res<ButtonInput<KeyCode>>,
		mouse_input: Res<AccumulatedMouseMotion>,
		query: Query<(&PlayerInput, Option<&Transform>, &mut MovementInput)>,
	) {
		for (player_input, xform, mut target) in query {
			let (movement, look) = match player_input.source {
				InputSource::KeyboardMouse => {
					// -z Forwards
					// +z Backwards
					// -x Left
					// +x Right
					let mut movement = Vec3::ZERO;
					if key_input.pressed(KeyCode::KeyW) { movement += Vec3::NEG_Z };
					if key_input.pressed(KeyCode::KeyS) { movement += Vec3::Z };
					if key_input.pressed(KeyCode::KeyA) { movement += Vec3::NEG_X };
					if key_input.pressed(KeyCode::KeyD) { movement += Vec3::X };
					movement = movement.normalize_or_zero();

					if let Some(xform) = xform {
						movement = xform.rotation.mul_vec3(movement);
					}

					let look = Vec3::new(
						mouse_input.delta.y,
						mouse_input.delta.x,
						0.0,
					);

					(movement, look * player_input.look_sensitivity)
				},
				InputSource::Gamepad(id) => todo!("Gamepads"),
				InputSource::Disabled => (Vec3::ZERO, Vec3::ZERO),
			};

			target.movement = movement; //? Gets overwritten.
			target.look += look; //? Gets accumulated.
		}
	}

	fn post_update(
		query: Query<&mut MovementInput, With<PlayerInput>>,
	) {
		for mut target in query {
			target.movement = Vec3::ZERO;
			target.look = Vec3::ZERO;
		}
	}
}
