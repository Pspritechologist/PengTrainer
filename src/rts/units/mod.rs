use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use tracing::instrument;
use bevy_egui::egui;

use crate::debug::PrototypeMaterial;

pub fn plugin(app: &mut App) {
	app
		
	;
}

#[derive(Debug, Clone, Default, Resource)]
pub struct UnitList(Vec<UnitData>);
impl UnitList {
	pub fn units(&self) -> impl IntoIterator<Item = &UnitData> {
		self.0.as_slice()
	}

	pub fn append(&mut self, data: UnitData) {
		self.0.push(data);
	}

	pub fn extend(&mut self, data: impl IntoIterator<Item = UnitData>) {
		self.0.extend(data);
	}
}

#[derive(Debug, Clone, Reflect)]
#[reflect(from_reflect = false)]
pub struct UnitData {
	name: String,
	icon: Option<u8>,
	#[reflect(ignore)]
	spawner: fn(&mut World),
}
impl Default for UnitData {
	fn default() -> Self {
		Self { icon: default(), name: default(), spawner: |_| () }
	}
}
impl UnitData {
	pub fn new(name: impl Into<String>, spawner: fn(&mut World)) -> Self {
		Self { name: name.into(), icon: None, spawner }
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn spawner(&self) -> impl Command {
		self.spawner
	}
}
