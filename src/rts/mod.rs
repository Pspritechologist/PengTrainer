use bevy::{math::bounding::Aabb2d, pbr::{Atmosphere, ScatteringMedium}, prelude::*};
use bevy_enhanced_input::prelude::*;

use units::UnitList;

mod cursor;
mod units;
mod unit_browser;

pub fn plugin(app: &mut App) {
	app
	.add_plugins(bevy_rts_camera::RtsCameraPlugin)
	.add_plugins(cursor::plugin)
	.add_plugins(units::plugin)
	.add_plugins(unit_browser::plugin)

	.add_systems(FixedUpdate, rts_update)
	.add_observer(select_unit)
	;
}

slotmap::new_key_type! {
	pub struct UnitId;
}

#[tracing::instrument(skip(cmds, assets))]
pub fn spawn_rts<'a>(cmds: &'a mut Commands, assets: &AssetServer, map_bounds: Aabb2d) -> EntityCommands<'a> {
	let rts_camera = cmds.spawn((
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
		Atmosphere::earthlike(assets.add(default())),
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
	)).id();

	let rts_cursor = cursor::Cursor::spawn(cmds, assets).id();

	cmds.queue(move |world: &mut World| {
		if let Some(state) = world.get_resource::<RtsState>() {
			error!("Spawning a new RTS player while RtsState was already populated.");
			let RtsState { rts_camera, rts_cursor, ..} = *state;
			_ = world.try_despawn(rts_camera);
			_ = world.try_despawn(rts_cursor);
		}
		
		world.insert_resource(RtsState {
			rts_camera,
			rts_cursor,
			selected_unit: None,
		});
	});

	cmds.entity(rts_camera)
}

#[derive(Debug, Clone, Copy, Event)]
pub struct SelectUnit(Option<UnitId>);

#[derive(Debug, Clone, Resource)]
struct RtsState {
	selected_unit: Option<UnitId>,
	rts_camera: Entity,
	rts_cursor: Entity,
}

fn rts_update(mut cmds: Commands, mut state: If<ResMut<RtsState>>, units: Res<UnitList>) {
	let state = &mut **state;

	if let Some(idx) = state.selected_unit {

	}
}

#[tracing::instrument(skip_all)]
fn select_unit(ev: On<SelectUnit>, mut state: If<ResMut<RtsState>>) {
	state.selected_unit = ev.0;
}
