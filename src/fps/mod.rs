use bevy::prelude::*;
use avian3d::prelude::*;
use tracing::instrument;

pub struct FpsPlayerPlugin;
impl Plugin for FpsPlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, Floater::update_velocity)
		   .add_systems(Update, Floater::update_torque);
	}
}

#[derive(Clone, Copy, Debug, Component, Reflect)]
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
	fn update_torque(floaters: Query<(Forces, &Floater)>) {
		for (mut forces, floater) in floaters {
			let current_rot = forces.rotation();
			let to_goal = current_rot.mul_vec3(Vec3::Y);
			
			let rot_axis = to_goal.cross(Vec3::Y).normalize();
			let rot_angle = to_goal.angle_between(Vec3::Y);

			let angular_vel = forces.angular_velocity();

			let torque = (rot_axis * (rot_angle * floater.upright_strength)) - (angular_vel * floater.upright_dampner);

			if torque.is_nan() {
				trace!("Skipping NaN torque: {torque}");
				continue;
			}

			trace!("Applying torque ({rot_axis} * {rot_angle} * {}) - ({angular_vel} * {}) = {torque}", floater.upright_strength, floater.upright_dampner);

			forces.apply_torque(torque);
		}
	}

	#[instrument(skip_all)]
	fn update_velocity(
		mut gizmos: Gizmos,
		spatial: SpatialQuery,
		floaters: Query<(
			Entity,
			&Floater,
			&mut ConstantForce,
		)>,
		mut forces: Query<Forces>,
	) {
		let down = Vec3::NEG_Y;

		for (floater_ent, &floater, mut force) in floaters {
			let xform = forces.get_mut(floater_ent).unwrap();

			let global_pos = **xform.position();
			let vel = xform.linear_velocity();

			let filter = SpatialQueryFilter::from_excluded_entities([floater_ent]);
			let hit = spatial.cast_ray(global_pos, Dir3::NEG_Y, floater.desired_height, false, &filter);
			let Some(hit) = hit else {
				force.0 = Vec3::ZERO;
				gizmos.ray(global_pos, Vec3::NEG_Y * floater.desired_height, LinearRgba::WHITE.with_alpha(0.1));
				continue;
			};

			let ground = forces.get_mut(hit.entity).ok();

			let ground_vel = ground.as_ref().map_or(Vec3::ZERO, |f| f.linear_velocity());

			let floater_dir_vel = down.dot(vel);
			let ground_dir_vel = down.dot(ground_vel);
			let relative_vel = ground_dir_vel - floater_dir_vel;

			let height_diff = hit.distance - floater.desired_height;
			let spring_force = (height_diff * floater.spring_strength) - (relative_vel * -floater.spring_damp);

			force.0 = down * spring_force;

			trace!("Applying force {} at {global_pos}", down * spring_force);

			if let Some(mut forces) = ground {
				let push_force = down * -spring_force;
				forces.apply_force_at_point(push_force, hit.normal);
			}

			// Draw a line the length of the Raycast, coloured based on the force being used.
			let max_force = floater.spring_strength * floater.desired_height;
			let force_amount = -spring_force / max_force;
			let alpha = (force_amount * 0.9) + 0.1;

			gizmos.ray(global_pos, Vec3::NEG_Y * floater.desired_height, LinearRgba::RED.with_alpha(alpha));
		}
	}
}
