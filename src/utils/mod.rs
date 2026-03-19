use bevy::{app::{App, PostUpdate}, ecs::schedule::IntoScheduleConfigs, transform::TransformSystems};

pub use transform_prop::*;

mod transform_prop;

pub struct UtilsPlugin;
impl bevy::prelude::Plugin for UtilsPlugin {
	fn build(&self, app: &mut App) {
		app
			// Xform prop.
			.add_systems(PostUpdate, transform_prop::update.after(TransformSystems::Propagate))
		;
	}
}
