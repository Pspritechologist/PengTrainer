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
	pub movement: Vec3,
	pub look: Vec3,
}
