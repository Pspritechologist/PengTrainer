use bevy::prelude::*;
use avian3d::prelude::*;

pub fn plugin(app: &mut App) {
	app
		.add_systems(Update, Cursor::update)
	;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Cursor {
	
}
impl Cursor {
	pub fn spawn<'a>(cmds: &'a mut Commands, assets: Res<AssetServer>) -> EntityCommands<'a> {
		cmds.spawn((
			Self { },
			Mesh3d(assets.add(Torus::new(0.2, 0.62).into())),
			(Sensor, Collider::cylinder(0.5, 0.3)),
		))
	}

	fn update(
		spatial: SpatialQuery,
	) {
		// spatial.
	}
}
