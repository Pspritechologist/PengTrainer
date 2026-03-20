use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use tracing::instrument;

use crate::debug::PrototypeMaterial;
use crate::movement::{FloatMovement, Floater, MovementInput};
use crate::utils::{TransformPropagate, TransformPropagateTo};

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
	fn build(&self, app: &mut App) {
		app.add_input_context::<FpsPlayerInput>()
			.add_systems(FixedPostUpdate, FpsPlayerInput::post_update)
			.add_observer(FpsPlayerInput::on_move_input)
			.add_observer(FpsPlayerInput::on_look_input)
			;
	}
}

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

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct FpsPlayerInput {
	pub look_sensitivity: f32,
	pub transform_target: Option<Entity>,
}
impl Default for FpsPlayerInput {
	fn default() -> Self {
		Self {
			look_sensitivity: 0.01,
			transform_target: None,
		}
	}
}

impl FpsPlayerInput {
	pub fn with_xform_target(mut self, target: Entity) -> Self {
		self.transform_target = Some(target);
		self
	}

	#[instrument(skip_all)]
	fn on_move_input(
		move_input: On<Fire<Movement>>,
		query: Query<(Entity, &FpsPlayerInput, &mut MovementInput)>,
		xforms: Query<&Transform>,
	) {
		let Ok((ent, player_input, mut target)) = query.get_inner(move_input.context) else {
			return;
		};
		
		debug!("Move input: {}", move_input.value);

		// -z Forwards, +z Backwards, -x Left, +x Right
		let mut movement = move_input.value.xxy() * Vec3::new(1., 0., -1.);

		let xform_target = player_input.transform_target.unwrap_or(ent);
		let target_xform = match xforms.get(xform_target) {
			Ok(xform) => xform,
			Err(e) => return warn!("Failed to get Entity of `target_xform`: {e}"),
		};

		movement = target_xform.rotation.mul_vec3(movement);

		target.move_direction = movement; //? Gets overwritten.
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

		target.look_target += look; //? Gets accumulated.
	}

	fn post_update(
		query: Query<&mut MovementInput, (With<FpsPlayerInput>, Changed<MovementInput>)>,
	) {
		for mut target in query {
			target.move_direction = Vec3::ZERO;
			target.look_target = Vec3::ZERO;
		}
	}
}


#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Movement;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Look;
