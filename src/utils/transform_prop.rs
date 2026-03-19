use bevy::prelude::*;

#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Component, Debug, Clone, Default)]
#[relationship_target(relationship = TransformPropagateTo, linked_spawn)]
pub struct TransformPropagateFrom(Vec<Entity>);

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component, Debug, Clone, Default)]
#[relationship(relationship_target = TransformPropagateFrom)]
pub struct TransformPropagateTo {
	#[relationship]
	master: Entity,
	pub translation: Vec3,
	pub rotation: Quat,
	pub scale: Vec3,
}

impl Default for TransformPropagateTo {
	fn default() -> Self {
		Self::full()
	}
}

impl TransformPropagateTo {
	pub fn none() -> Self {
		Self {
			master: Entity::PLACEHOLDER,
			translation: Vec3::ZERO,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
		}
	}
	pub fn full() -> Self {
		Self {
			master: Entity::PLACEHOLDER,
			translation: Vec3::ONE,
			rotation: Quat::IDENTITY,
			scale: Vec3::ONE,
		}
	}

	pub fn without_translation(mut self) -> Self {
		self.translation = Vec3::ZERO;
		self
	}
	pub fn without_rotation(mut self) -> Self {
		self.rotation = Quat::IDENTITY;
		self
	}
	pub fn without_scale(mut self) -> Self {
		self.scale = Vec3::ONE;
		self
	}

	pub fn with_translation(mut self, translation: Vec3) -> Self {
		self.translation = translation;
		self
	}
	pub fn with_rotation(mut self, rotation: Quat) -> Self {
		self.rotation = rotation;
		self
	}
	pub fn with_scale(mut self, scale: Vec3) -> Self {
		self.scale = scale;
		self
	}

	pub fn with_full_translation(self) -> Self {
		self.with_translation(Vec3::ONE)
	}
	pub fn with_full_rotation(self) -> Self {
		self.with_rotation(Quat::IDENTITY)
	}
	pub fn with_full_scale(self) -> Self {
		self.with_scale(Vec3::ONE)
	}
}

pub fn update(
	masters: Populated<(Entity, &TransformPropagateFrom), Changed<Transform>>,
	mut params: ParamSet<(
		Query<&Transform>,
		Query<(&TransformPropagateTo, &mut Transform)>,
	)>
) {
	for (master, master_from) in masters {
		let master_xform = *params.p0().get(master).unwrap();
		let mut xforms = params.p1();
		let mut slaves = xforms.iter_many_mut(&master_from.0);
		while let Some((slave_to, mut slave_xform)) = slaves.fetch_next() {
			*slave_xform = Transform {
				translation: master_xform.translation * slave_to.translation,
				rotation: master_xform.rotation * slave_to.rotation,
				scale: master_xform.scale * slave_to.scale,
			};
		}
	}
}
