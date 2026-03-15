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
		.register_type::<FlickeringLight>()
		.add_systems(PostUpdate, Ball::handle_spawn)
		.add_systems(FixedUpdate, FlickeringLight::update)
		;
	}
}

#[point_class(base(PointLight), group("light"))]
#[derive(Debug, Default, Component)]
struct FlickeringLight {
	/// Chance for the light to turn off every second.
	chance: f64,
	/// Time the light will remain off when flickering in seconds.
	time_off: f64,
	#[class(ignore)]
	off_time: Option<f64>,
	#[class(ignore)]
	last_intensity: f32,
}
impl FlickeringLight {
	fn update(
		query: Query<(&mut FlickeringLight, &mut PointLight)>,
		time: Res<Time>,
	) {
		for (mut flickering, mut light) in query {
			if let Some(off_time) = flickering.off_time.as_mut() {
				*off_time -= time.delta_secs_f64();

				if *off_time <= 0.0 {
					flickering.off_time = None;
					light.intensity = flickering.last_intensity;
				}

				continue
			}

			let chance = flickering.chance * time.delta_secs_f64();
			if chance > fastrand::f64() {
				flickering.last_intensity = light.intensity;
				light.intensity = 0.0;
				flickering.off_time = Some(flickering.time_off);
			}
		}
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

		let color = match name {
			Some(name) => name.as_str().color(),
			None => {
				entity.get::<Transform>().map_or(Vec3::ZERO, |xform| xform.translation).color()
			},
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
pub struct Ball {
	/// Nyooom.
	velocity: Vec3,
}

impl Ball {
	fn handle_spawn(
		mut commands: Commands,
		entities: Query<(Entity, &Ball, Option<&Transform>), Added<Ball>>,
		mut meshes: ResMut<Assets<Mesh>>,
	) {
		let mesh = meshes.add(Mesh::from(Sphere::new(0.5)));

		for (entity, ball, xform) in entities {
			let pos = xform.map_or(Vec3::ZERO, |xform| xform.translation);

			commands.entity(entity).insert((
				LinearVelocity(ball.velocity),
				RigidBody::Dynamic,
				Collider::sphere(0.5),
				Mesh3d(mesh.clone()),
				debug::PrototypeMaterial::new(pos),
			));
		}
	}
}
