use bevy::prelude::*;

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
	fn build(&self, app: &mut App) {
		app
			.init_resource::<InspectorToggle>()
			.add_systems(PostUpdate, toggle_inspector)
			.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
			.add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new().run_if(|toggle: Res<InspectorToggle>| toggle.0));
	}
}

#[derive(Clone, Copy, Default, Resource)]
struct InspectorToggle(bool);

fn toggle_inspector(
	mut toggle: ResMut<InspectorToggle>,
	input: Res<ButtonInput<KeyCode>>,
) {
	if input.just_pressed(KeyCode::F1) {
		toggle.0 = !toggle.0;
	}
}
