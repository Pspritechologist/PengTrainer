use bevy::app::{App, FixedUpdate};

pub use transform_prop::*;

mod transform_prop;

pub struct UtilsPlugin;
impl bevy::prelude::Plugin for UtilsPlugin {
	fn build(&self, app: &mut App) {
		app
			// Xform prop.
			.add_systems(FixedUpdate, transform_prop::update)
		;
	}
}
