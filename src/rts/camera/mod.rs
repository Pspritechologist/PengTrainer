use bevy::prelude::*;

pub struct RtsCameraPlugin;
impl Plugin for RtsCameraPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, on_update);
	}
}

#[derive(Debug, Clone, Copy, Component)]
pub struct RtsCamera {

}

#[derive(Debug, Clone, Copy, Component)]
struct RtsCameraState {

}

fn on_update(
	query: Query<(&mut Transform, &mut RtsCameraState), With<RtsCamera>>,
) {

}
