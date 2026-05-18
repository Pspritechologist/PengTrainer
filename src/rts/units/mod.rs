use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use slotmap::SlotMap;
use tracing::instrument;
use bevy_egui::egui;

use crate::debug::PrototypeMaterial;
use super::UnitId;

pub fn plugin(app: &mut App) {
	app
		.add_observer(SpawnUnit::on_spawn)
	;
}

#[derive(Debug, Clone, Default, Resource, Deref, DerefMut)]
pub struct UnitList(SlotMap<UnitId, UnitData>);
impl UnitList {
	pub fn units(&self) -> impl IntoIterator<Item = (UnitId, &UnitData)> {
		self.0.iter()
	}

	pub fn append(&mut self, data: UnitData) -> UnitId {
		self.0.insert(data)
	}

	pub fn extend(&mut self, data: impl IntoIterator<Item = UnitData>) {
		let data = data.into_iter();
		self.0.reserve(data.size_hint().0);
		for data in data {
			self.append(data);
		}
	}
}

#[derive(Debug, Clone, Reflect)]
#[reflect(from_reflect = false)]
pub struct UnitData {
	name: String,
	icon: Option<u8>,
	#[reflect(ignore)]
	spawner: fn(&mut EntityWorldMut) -> Result,
}
impl UnitData {
	pub fn new(name: impl Into<String>, spawner: fn(&mut EntityWorldMut) -> Result) -> Self {
		Self { name: name.into(), icon: None, spawner }
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Debug, Clone, Event)]
pub struct SpawnUnit {
	unit: UnitId,
	transform: Transform,
}
impl SpawnUnit {
	pub fn new(unit: UnitId, transform: Transform) -> Self {
		Self { unit, transform }
	}

	#[tracing::instrument(skip_all)]
	fn on_spawn(ev: On<Self>, units: Res<UnitList>, mut cmds: Commands) {
		let Some(data) = units.0.get(ev.unit) else {
			error!("Tried to spawn unit with invalid id: {:?}", ev.unit);
			return;
		};

		info!("Spawning unit: {}", data.name());

		let spawner = data.spawner;
		let xform = ev.transform;
		cmds.queue(move |world: &mut World| {
			spawner(&mut world.spawn(xform))
		});
	}
}
