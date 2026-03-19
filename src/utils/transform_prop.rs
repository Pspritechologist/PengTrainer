use bevy::prelude::*;

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component, Debug, Clone, Default)]
#[relationship_target(relationship = TransformPropagateTo, linked_spawn)]
#[require(Transform)]
pub struct TransformPropagateFrom(Vec<Entity>);

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component, Debug, Clone)]
#[relationship(relationship_target = TransformPropagateFrom)]
#[require(TransformPropagate, Transform)]
pub struct TransformPropagateTo(Entity);

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component, Debug, Clone, Default)]
pub struct TransformPropagate {
	pub translation: Option<Vec3>,
	pub rotation: Option<Quat>,
	pub scale: Option<Vec3>,
}

impl Default for TransformPropagate {
	fn default() -> Self {
		Self::full()
	}
}
impl TransformPropagate {
	pub fn none() -> Self {
		Self {
			translation: None,
			rotation: None,
			scale: None,
		}
	}
	pub fn full() -> Self {
		Self {
			translation: Some(Vec3::ZERO),
			rotation: Some(Quat::IDENTITY),
			scale: Some(Vec3::ZERO),
		}
	}

	pub fn without_translation(mut self) -> Self {
		self.translation = None;
		self
	}
	pub fn without_rotation(mut self) -> Self {
		self.rotation = None;
		self
	}
	pub fn without_scale(mut self) -> Self {
		self.scale = None;
		self
	}

	pub fn with_translation_offset(mut self, translation: Vec3) -> Self {
		self.translation = Some(translation);
		self
	}
	pub fn with_rotation_offset(mut self, rotation: Quat) -> Self {
		self.rotation = Some(rotation);
		self
	}
	pub fn with_scale_offset(mut self, scale: Vec3) -> Self {
		self.scale = Some(scale);
		self
	}

	pub fn with_full_translation(self) -> Self {
		self.with_translation_offset(Vec3::ZERO)
	}
	pub fn with_full_rotation(self) -> Self {
		self.with_rotation_offset(Quat::IDENTITY)
	}
	pub fn with_full_scale(self) -> Self {
		self.with_scale_offset(Vec3::ZERO)
	}
}

pub fn update(
	mut commands: Commands,
	masters: Populated<(&TransformPropagateFrom, &Transform), Changed<Transform>>,
	slaves: Query<(Entity, &TransformPropagate, &Transform)>,
) {
	for (master_from, master_xform) in masters {
		let mut slaves = slaves.iter_many(&master_from.0);
		while let Some((slave, slave_to, slave_xform)) = slaves.fetch_next() {
			commands.entity(slave).insert(Transform {
				translation: slave_to.translation.map(|o| master_xform.translation + o).unwrap_or(slave_xform.translation),
				rotation: slave_to.rotation.map(|o| master_xform.rotation + o).unwrap_or(slave_xform.rotation),
				scale: slave_to.scale.map(|o| master_xform.scale + o).unwrap_or(slave_xform.scale),
			});
		}
	}
}
