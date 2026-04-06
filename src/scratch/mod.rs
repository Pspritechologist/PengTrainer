//! Somewhere for code to live temporarily while it's worked on, before it's cemented and a place for it is found.
//! Mostly just prevents making a bunch of temporary modules while committing stuff.

use bevy::{camera::visibility::NoFrustumCulling, prelude::*};
use avian3d::prelude::LinearVelocity;
use crate::movement;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app
			.add_systems(FixedUpdate, (throw_balls, FollowEntity::update))
			.add_systems(Update, daylight_cycle)
			.add_systems(PostStartup, setup_env)
		;
	}
}

fn throw_balls(entities: Query<&mut LinearVelocity, With<crate::trenchbroom::Ball>>) {
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

fn daylight_cycle(mut sun_xform: Single<&mut Transform, With<DirectionalLight>>, time: Res<Time>) {
	sun_xform.rotate_x(-time.delta_secs() * std::f32::consts::PI / (10.0 * 60.0));
}

fn setup_env(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.insert_resource(ClearColor(Color::BLACK));
	commands.insert_resource(GlobalAmbientLight::NONE);

	commands.spawn((
		DirectionalLight {
			illuminance: 32000.0,
			shadows_enabled: true,
			..Default::default()
		},
		Transform::from_xyz(0.8, 2., -1.).looking_at(Vec3::ZERO, Vec3::Y),
		bevy::light::VolumetricLight,
		bevy::light::CascadeShadowConfigBuilder {
			..Default::default()
		}.build(),
	));

	commands.spawn((
		bevy::light::FogVolume {
			absorption: 1.0,
			density_factor: 1.0,
			density_texture: Some(asset_server.load("bunny.ktx2")),
			scattering: 1.0,
			..Default::default()
		},
		Transform::from_xyz(9.5, 3.3, 0.0).with_scale(Vec3::new(6.0, 6.0, 6.0)),
		NoFrustumCulling,
	));
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
