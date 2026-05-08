use bevy::{math::bounding::Aabb2d, pbr::{Atmosphere, ScatteringMedium}, prelude::*};
use bevy_enhanced_input::prelude::*;

mod units;
mod unit_browser;

pub fn plugin(app: &mut App) {
	app
		.add_plugins(bevy_rts_camera::RtsCameraPlugin)
		.add_plugins(units::plugin)
		.add_plugins(unit_browser::plugin)
	;
}

pub fn spawn_rts<'a>(cmds: &'a mut Commands, scattering_medium: Handle<ScatteringMedium>, map_bounds: Aabb2d) -> EntityCommands<'a> {
	cmds.spawn((
		Name::new("RtsPlayer"),
		Camera3d::default(),
		bevy_rts_camera::RtsCamera {
			// height_min: 0.1,
			height_max: 80.0,
			bounds: map_bounds,
			..Default::default()
		},
		bevy_rts_camera::RtsCameraControls {
			key_up: KeyCode::KeyW,
			key_down: KeyCode::KeyS,
			key_left: KeyCode::KeyA,
			key_right: KeyCode::KeyD,
			edge_pan_width: 0.0,
			lock_on_rotate: true,
			zoom_sensitivity: 0.25,
			button_drag: Some(MouseButton::Right),
			..Default::default()
		},
		Transform::from_translation(Vec3::new(0., 0., -0.12)),
		Projection::Perspective(PerspectiveProjection {
			fov: 45.0f32.to_radians(),
			..Default::default()
		}),
		Atmosphere::earthlike(scattering_medium),
		bevy::camera::Exposure { ev100: 13.0 },
		bevy::post_process::bloom::Bloom::NATURAL,
		bevy::light::AtmosphereEnvironmentMapLight::default(),
		bevy::light::VolumetricFog {
			ambient_intensity: 0.0,
			..Default::default()
		},
		Msaa::Off,
		bevy::anti_alias::fxaa::Fxaa::default(),
		bevy::pbr::ScreenSpaceReflections::default(),
	))
}
