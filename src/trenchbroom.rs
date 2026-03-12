use bevy::prelude::*;
use bevy_trenchbroom::{config::MapFileFormat, prelude::*};
use bevy_trenchbroom_avian::AvianPhysicsBackend;
use avian3d::prelude::{Collider, LinearVelocity, RigidBody};

use crate::debug::{PrototypeMaterialAsset, PrototypeMaterial};
use bevy_trenchbroom::config::TextureLoadView;
use bevy::tasks::BoxedFuture;
use std::sync::Arc;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((TrenchBroomPhysicsPlugin::new(AvianPhysicsBackend), TrenchBroomPlugins(TrenchBroomConfig::new("PengTrainerBevy")
			.file_formats([MapFileFormat::Valve])
			// .load_loose_texture_fn(load::default_load_loose_texture)
			.default_solid_scene_hooks(|| SceneHooks::new().convex_collider()))))
		.add_systems(PostUpdate, Ball::handle_spawn)
		.register_type::<Ball>();
	}
}

type LoadLooseTextureFn = dyn for<'a, 'b> Fn(TextureLoadView<'a, 'b>) -> BoxedFuture<'a, Handle<GenericMaterial>> + Send + Sync;

pub fn default_load_loose_texture(f: Arc<LoadLooseTextureFn>) -> Arc<LoadLooseTextureFn> {
	Arc::new(move |view| {
		let f = f.clone();
		Box::pin(async move {
			if let Some(name) = view.name.strip_prefix("Proto_") {
				let mat = PrototypeMaterialAsset::new(name, view.asset_server);
				let handle = view.asset_server.add(mat);

				return view.load_context.add_labeled_asset(format!("Material_{}", view.name), GenericMaterial::new(handle));
			}

			f(view).await
		})
	})
}

#[point_class]
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

			let [x, y, z] = [
				pos.x.trunc(),
				pos.y.trunc(),
				pos.z.trunc(),
			];

			let [xf, yf, zf] = [
				((pos.x.fract() - x) * 100.0) as i64,
				((pos.y.fract() - y) * 100.0) as i64,
				((pos.z.fract() - z) * 100.0) as i64,
			];

			let source = [(x as i64, xf), (y as i64, yf), (z as i64, zf)];

			commands.entity(entity).insert((
				LinearVelocity(ball.velocity),
				RigidBody::Dynamic,
				Collider::sphere(0.5),
				Mesh3d(mesh.get_or_insert_with(|| meshes.add(Mesh::from(Sphere::new(0.5)))).clone()),
				PrototypeMaterial::new(crate::debug::HashSource(source)),
			));
		}
	}
}
