use std::ops::Neg;

use bevy::prelude::*;
use avian3d::prelude::{forces::ForcesItem, *};
use tracing::instrument;

pub mod player;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(player::PlayerInputPlugin)
			.add_systems(FixedUpdate, (
				Floater::update_velocity,
				Floater::update_torque_upright,
				Floater::update_torque_target,
			));
	}
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct MovementInput {
	/// The direction to be moved towards, as an absolute vector.\
	/// Note that this is *not* relative to the Entity.
	pub move_direction: Vec3,
	/// The position to be looked towards, as an absolute vector.\
	/// Note that this is *not* relative to the Entity.
	pub look_target: Vec3,
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[require(MovementInput)]
pub struct FloatMovement {
	pub max_speed: f32,
	pub acceleration: f32,
	pub max_accel_force: f32,
	/// How fast one can change directions.\ 
	/// Added to acceleration as catchup when changing directions.\
	/// > "This baby turns on a dime!"
	pub dimeyness: f32,
	goal_velocity: Vec3,
}
impl Default for FloatMovement {
	fn default() -> Self {
		Self {
			max_speed: 6.0,
			acceleration: 20.0,
			max_accel_force: 100.0,
			dimeyness: 20.0,
			goal_velocity: Vec3::ZERO,
		}
	}
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[require(
	RigidBody::Dynamic,
	ConstantForce,
	ConstantTorque,
)]
pub struct Floater {
	pub desired_height: f32,
	pub spring_strength: f32,
	pub spring_damp: f32,

	pub upright_strength: f32,
	pub upright_dampner: f32,
}
impl Default for Floater {
	fn default() -> Self {
		Self {
			desired_height: 4.0,
			spring_strength: 12.0,
			spring_damp: 1.5,
			upright_strength: 6.0,
			upright_dampner: 0.3,
		}
	}
}
impl Floater {
	#[instrument(skip_all)]
	fn update_torque_upright(floaters: Populated<(Forces, &Floater), Without<MovementInput>>) {
		for (mut forces, floater) in floaters {
			let current_rot = forces.rotation();
			let to_goal = current_rot.mul_vec3(Vec3::Y);
			
			let axis = to_goal.cross(Vec3::Y).normalize();
			let angle = to_goal.angle_between(Vec3::Y);
			
			floater.update_torque_inner(&mut forces, axis, angle);
		}
	}

	#[instrument(skip_all)]
	fn update_torque_target(floaters: Populated<(
		Forces,
		&GlobalTransform,
		&Floater,
		&MovementInput,
	)>) {
		for (mut forces, glob_xform, floater, target) in floaters {
			let look_at_pos = target.look_target;
			let look_from_pos = glob_xform.translation();
			let to_target = (look_at_pos - look_from_pos).normalize();
			
			let desired_rot = Transform::default().looking_to(to_target, Vec3::Y).rotation;
			let current_rot = *forces.rotation();
			let rot_diff = desired_rot * current_rot.conjugate();
			let rot_diff = if rot_diff.w < 0.0 { -rot_diff } else { rot_diff };
			let (axis, angle) = rot_diff.to_axis_angle();

			floater.update_torque_inner(&mut forces, axis, angle);
		}
	}

	fn update_torque_inner(&self, forces: &mut ForcesItem, axis: Vec3, angle: f32) {
		let angular_vel = forces.angular_velocity();

		let mut torque = (axis * (angle * self.upright_strength)) - (angular_vel * self.upright_dampner);

		if torque.is_nan() {
			torque = Vec3::ZERO;
		}

		trace!("Applying torque ({axis} * {angle} * {}) - ({angular_vel} * {}) = {torque}", self.upright_strength, self.upright_dampner);

		forces.apply_torque(torque);
	}

	#[instrument(skip_all)]
	fn update_velocity(
		time: Res<Time>,
		mut gizmos: Gizmos,
		spatial: SpatialQuery,
		floaters: Query<(
			NameOrEntity,
			&Floater,
			&mut ConstantForce,
			Option<(&mut FloatMovement, &MovementInput)>,
		)>,
		mut forces: Query<Forces>,
	) {
		let down = Vec3::NEG_Y;

		for (floater_ent, &floater, mut force, movement_comps) in floaters {
			let xform = forces.get_mut(floater_ent.entity).unwrap();

			let global_pos = **xform.position();
			let velocity = xform.linear_velocity();

			let filter = SpatialQueryFilter::from_excluded_entities([floater_ent.entity]);
			let hit = spatial.cast_ray(global_pos, Dir3::NEG_Y, floater.desired_height, false, &filter);
			let Some(hit) = hit else {
				force.0 = Vec3::ZERO;
				gizmos.ray(global_pos, Vec3::NEG_Y * floater.desired_height, LinearRgba::WHITE.with_alpha(0.1));
				continue;
			};

			let ground = forces.get_mut(hit.entity).ok();

			let ground_vel = ground.as_ref().map_or(Vec3::ZERO, |f| f.linear_velocity());

			let floater_dir_vel = down.dot(velocity);
			let ground_dir_vel = down.dot(ground_vel);
			let relative_vel = ground_dir_vel - floater_dir_vel;

			let height_diff = hit.distance - floater.desired_height;
			let spring_force = (height_diff * floater.spring_strength) - (relative_vel * -floater.spring_damp);

			force.0 = down * spring_force;

			trace!("Applying force {} at {global_pos}", down * spring_force);

			if let Some(mut forces) = ground {
				let push_force = down * spring_force;
				trace!("Applying push force {push_force} at {} to Entity {}", hit.normal, hit.entity);
				forces.apply_force_at_point(-push_force, hit.normal + **forces.position());
			}

			// Draw a line the length of the Raycast, coloured based on the force being used.
			let max_force = floater.spring_strength * floater.desired_height;
			let force_amount = -spring_force / max_force;
			let alpha = (force_amount * 0.9) + 0.1;

			gizmos.ray(global_pos, Vec3::NEG_Y * floater.desired_height, LinearRgba::RED.with_alpha(alpha));

			if let Some((mut float_move, target)) = movement_comps {
				// The ideal velocity to have right now, based on our inputs.
				let desired_speed = target.move_direction * float_move.max_speed;
				let target_velocity = desired_speed + ground_vel;
				let accel_mod = (velocity.normalize_or_zero() - target_velocity.normalize_or_zero()).length();
				let acceleration = float_move.acceleration + (accel_mod * float_move.dimeyness);

				// Move our current 'goal velocity' towards the ideal velocity based on our acceleration.
				float_move.goal_velocity = float_move.goal_velocity.move_towards(
					desired_speed + ground_vel,
					acceleration * time.delta_secs(),
				);


				// Calculate the amount of force required to move from our current velocity to our goal velocity in a single tick.
				let velocity_difference = float_move.goal_velocity - velocity;
				let needed_force = velocity_difference / time.delta_secs();
				// ... Limited by the maximum force of our acceleration.
				let needed_force = needed_force.clamp_length_max(float_move.max_accel_force);

				debug!("Applying movement accel: {needed_force} \n\tgoal: {}\n\tvel: {velocity}\n\tground_vel: {ground_vel})", float_move.goal_velocity);

				forces.get_mut(floater_ent.entity).unwrap().apply_linear_acceleration(needed_force);
			}
		}
	}
}
