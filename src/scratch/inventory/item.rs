use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Debug, Clone, Component, Default)]
pub struct CanBePickedUp {
	#[reflect(ignore)]
	pub apply_item_bundle: fn(EntityCommands),
}
impl Default for CanBePickedUp {
	fn default() -> Self {
		Self {
			apply_item_bundle: |_| (),
		}
	}
}
impl CanBePickedUp {
	pub fn new(apply_item_bundle: fn(EntityCommands)) -> Self {
		Self { apply_item_bundle }
	}
}

#[derive(Debug, Clone, Copy, Component, Reflect, Deref, DerefMut)]
#[reflect(Debug, Clone, Default, Component)]
pub struct ItemWeight(pub u32);
impl Default for ItemWeight {
	fn default() -> Self { Self(5) }
}
