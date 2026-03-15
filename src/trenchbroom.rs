use bevy::prelude::*;
use bevy_trenchbroom::{anyhow, class::{QuakeClassSpawnView, builtin::FuncGeneric}, config::MapFileFormat, prelude::*};
use bevy_trenchbroom_avian::AvianPhysicsBackend;
use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use crate::debug::{self, ColorSource};

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			TrenchBroomPhysicsPlugin::new(AvianPhysicsBackend),
			TrenchBroomPlugins(TrenchBroomConfig::new("PengTrainerBevy")
				.file_formats([MapFileFormat::Quake2Valve])
				.icon(Some(include_bytes!("../icon/32x.png").into()))
				.global_transform_application(false)
				.default_solid_scene_hooks(|| SceneHooks::new().convex_collider()))
			))
		.add_systems(PostUpdate, Ball::handle_spawn);
	}
}

#[base_class(hooks(SceneHooks::new().push(Self::hook)))]
#[derive(Debug, Default, Component)]
struct PrototypeMaterial {
	/// The identifier of this brush, such as 'wall' or 'floor'.\
	/// This can be anything and will be used to generate the brush's colour.
	name: Option<String>,
}
impl PrototypeMaterial {
	fn hook(view: &mut QuakeClassSpawnView) -> anyhow::Result<()> {
		let mut entity = view.world.entity_mut(view.entity);

		let name = entity.get::<PrototypeMaterial>().unwrap().name.as_ref();

		let get_pos = || {
			let Some(xform) = entity.get::<Transform>() else {
				return Ok(Vec3::ZERO)
			};

			let pos = xform.translation;
			Ok::<_, anyhow::Error>(pos)
		};

		let color = match name {
			Some(name) => name.as_str().color(),
			None => get_pos()?.color(),
		};

		entity.insert(debug::PrototypeMaterial::new(color));

		Ok(())
	}
}

#[solid_class(base(FuncGeneric, PrototypeMaterial))]
#[derive(Debug, Default, Component)]
/// A prototype brush, to be assigned a random colour based one
/// the name if one is provided, or its position otherwise.
struct PrototypeBrush;

#[point_class(base(Transform))]
#[derive(Default, Component)]
/// A parkin ball
struct Ball {
	/// Nyooom.
	velocity: Vec3,
}

impl Ball {
	fn handle_spawn(
		mut commands: Commands,
		entities: Query<(Entity, &Ball, Option<&Transform>), Changed<Ball>>,
		mut meshes: ResMut<Assets<Mesh>>,
	) {
		let mut mesh = None;

		for (entity, ball, xform) in entities {
			let pos = xform.map_or(Vec3::ZERO, |xform| xform.translation);

			commands.entity(entity).insert((
				LinearVelocity(ball.velocity),
				RigidBody::Dynamic,
				Collider::sphere(0.5),
				Mesh3d(mesh.get_or_insert_with(|| meshes.add(Mesh::from(Sphere::new(0.5)))).clone()),
				debug::PrototypeMaterial::new(pos),
			));
		}
	}
}
