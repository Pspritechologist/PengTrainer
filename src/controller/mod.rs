use bevy::prelude::*;

pub mod input_handler;

pub struct Plugin;
impl bevy::prelude::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(input_handler::PlayerInputPlugin);
	}
}

#[derive(Debug, Clone, Copy, Default, Component, Reflect)]
pub struct MovementInput {
	/// The direction to be moved towards, as an absolute vector.\
	/// Note that this is *not* relative to the Entity.
	pub move_direction: Vec3,
	/// The position to be looked towards, as an absolute vector.\
	/// Note that this is *not* relative to the Entity.
	pub look_target: Vec3,
}
