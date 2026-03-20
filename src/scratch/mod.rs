//! Somewhere for code to live temporarily while it's worked on, before it's cemented and a place for it is found.
//! Mostly just prevents making a bunch of temporary modules while committing stuff.

use bevy::prelude::*;
use avian3d::prelude::LinearVelocity;
use crate::movement;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_systems(FixedUpdate, (throw_balls, FollowEntity::update));
	}
}

fn throw_balls(
	entities: Query<&mut LinearVelocity, With<crate::trenchbroom::Ball>>,
) {
	for mut vel in entities {
		if fastrand::f32() < 0.002 {
			vel.0 += Vec3::new(
				fastrand::f32() * 30.0 - 15.0,
				fastrand::f32() * 10.0 - 6.5,
				fastrand::f32() * 30.0 - 15.0,
			);
		}
	}
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[require(GlobalTransform, movement::MovementInput)]
pub struct FollowEntity(pub Entity);
impl FollowEntity {
	#[tracing::instrument(skip_all)]
	fn update(
		query: Query<(Entity, &FollowEntity, &mut movement::MovementInput)>,
		xforms: Query<&GlobalTransform>,
	) {
		for (ent, follow, mut target) in query {
			let Ok(target_pos) = xforms.get(follow.0).map(|x| x.translation()) else {
				warn!("Tried to follow Entity {} without a GlobalPosition Component", follow.0);
				continue
			};
			let pos = xforms.get(ent).unwrap().translation();

			let movement = (target_pos - pos).normalize();
			if movement.is_nan() {
				trace!("Entity {ent} is already at the target position");
				continue;
			}
			
			trace!("Following Entity {}: Target Pos: {target_pos}, Current Pos: {pos}, Movement: {movement}", follow.0);

			target.move_direction = movement;
		}
	}
}
