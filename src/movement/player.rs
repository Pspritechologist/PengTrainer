use bevy::pbr::{Atmosphere, ScatteringMedium};
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
			.add_observer(FpsPlayerInput::on_move_input)
			.add_observer(FpsPlayerInput::on_release_move_input)
			.add_observer(FpsPlayerInput::on_look_input)
			.add_systems(Update, FpsPlayerInput::handle_look);
	}
}

pub fn spawn_player(commands: &mut Commands, scattering_medium: Handle<ScatteringMedium>, meshes: &mut Assets<Mesh>) -> Entity {
	let camera = commands.spawn((
		Camera3d::default(),
		// bevy::camera::Exposure::from_physical_camera(bevy::camera::PhysicalCameraParameters {
		// 	aperture_f_stops: 1.0,
		// 	shutter_speed_s: 1.0 / 125.0,
		// 	sensitivity_iso: 100.0,
		// 	sensor_height: 0.01866,
		// }),
		Transform::from_translation(Vec3::new(0., 0., -0.12)),
		Projection::Perspective(PerspectiveProjection {
			fov: 90.0f32.to_radians(),
			..Default::default()
		}),
		Atmosphere::earthlike(scattering_medium),
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
			spring_strength: 48.0,
			spring_damp: 0.001,
			..Default::default()
		},
		FloatMovement::default(),
		FpsPlayerInput { look_sensitivity: 0.007, head, camera },
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
					(Binding::mouse_motion(), Negate::all()),
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
	pub head: Entity,
	pub camera: Entity,
}

impl FpsPlayerInput {
	#[instrument(skip_all)]
	fn on_move_input(
		move_input: On<Fire<Movement>>,
		query: Query<(&FpsPlayerInput, &mut MovementInput)>,
		xforms: Query<&Transform>,
	) {
		let Ok((player_input, mut target)) = query.get_inner(move_input.context) else {
			return;
		};
		
		debug!("Move input: {}", move_input.value);

		// -z Forwards, +z Backwards, -x Left, +x Right
		let mut movement = move_input.value.xxy() * Vec3::new(1., 0., -1.);

		let head_xform = match xforms.get(player_input.head) {
			Ok(xform) => xform,
			Err(e) => return warn!("Failed to get Entity of `target_xform`: {e}"),
		};

		movement = head_xform.rotation.mul_vec3(movement);

		target.move_direction = movement; //? Gets overwritten.
	}

	#[instrument(skip_all)]
	fn on_release_move_input(
		observer: On<Complete<Movement>>,
		query: Query<&mut MovementInput, With<FpsPlayerInput>>,
	) {
		let Ok(mut target) = query.get_inner(observer.context) else {
			return;
		};

		debug!("Move input released");

		target.move_direction = Vec3::ZERO;
	}

	#[instrument(skip_all)]
	fn on_look_input(
		look_input: On<Fire<Look>>,
		query: Query<&FpsPlayerInput>,
		mut xforms: Query<&mut Transform>,
	) {
		let Ok(player_input) = query.get_inner(look_input.context) else {
			return;
		};

		debug!("Look input: {}", look_input.value);

		xforms.get_mut(player_input.head).unwrap().rotate_y(look_input.value.x * player_input.look_sensitivity);
		
		let mut cam_xform = xforms.get_mut(player_input.camera).unwrap();
		
		let (_, mut cam_rot_x, _) = cam_xform.rotation.to_euler(EulerRot::YXZ);
		cam_rot_x += look_input.value.y * player_input.look_sensitivity;
		cam_xform.rotation = Quat::from_euler(EulerRot::XYZ, cam_rot_x.clamp(-89.9f32.to_radians(), 89.9f32.to_radians()), 0., 0.);
	}

	#[instrument(skip_all)]
	fn handle_look(
		query: Query<(&FpsPlayerInput, &mut MovementInput)>,
		xforms: Query<&GlobalTransform>,
	) {
		for (player_input, mut target) in query {
			let cam_glob_xform = match xforms.get(player_input.camera) {
				Ok(xform) => xform,
				Err(e) => return warn!("Failed to get Entity of `target_xform`: {e}"),
			};

			target.look_target = cam_glob_xform.translation() + *cam_glob_xform.forward();
		}
	}
}


#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Movement;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct Look;
