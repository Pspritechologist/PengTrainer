use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::{debug::PrototypeMaterial, utils::GameLayer};

pub fn plugin(app: &mut App) {
	app
		.add_input_context::<RtsPlayerInput>()
		.add_observer(PlaceUnit::on_place)

		.add_systems(Update, Cursor::update)
	;
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Cursor {
	active: bool,
}
impl Cursor {
	pub fn spawn<'a>(cmds: &'a mut Commands, assets: &AssetServer) -> EntityCommands<'a> {
		cmds.spawn((
			Self { active: false },
			RtsPlayerInput,
			Mesh3d(assets.add(Torus::new(0.60, 0.62).into())),
			PrototypeMaterial::new("cursor"),
			// (Sensor, Collider::cylinder(0.5, 0.3)),
			actions!(RtsPlayerInput[
				(Action::<PlaceUnit>::new(), bindings![MouseButton::Left]),
			]),
		))
	}

	#[tracing::instrument(skip_all)]
	fn update(
		mut cmds: Commands,
		state: Res<super::RtsState>,
		window: Single<&Window, With<bevy::window::PrimaryWindow>>,
		egui: bevy_egui::EguiContexts,
		cam_query: Query<(&Camera, &GlobalTransform)>,
		mut cursor_query: Populated<(&mut Cursor, &Visibility, &Transform)>,
		spatial: SpatialQuery,
		time: Res<Time>,
	) -> Result {
		let (mut cursor, &vis, &(mut cursor_xform)) = cursor_query.get_mut(state.rts_cursor).unwrap();

		let mut disable = || {
			cursor.active = false;
			if !matches!(vis, Visibility::Hidden) {
				cmds.entity(state.rts_cursor).insert(Visibility::Hidden);
			}
		};

		let is_pointer_in_ui = {
			let ctx = egui.ctx().unwrap();
			ctx.wants_pointer_input() || ctx.is_using_pointer()
		};
		
		if is_pointer_in_ui {
			disable();
			return Ok(());
		}

		let Some(cursor_pos) = window.cursor_position() else {
			disable();
			return Ok(());
		};

		let (cam, cam_xform) = cam_query.get(state.rts_camera).unwrap();
		let ray = cam.viewport_to_world(cam_xform, cursor_pos)?;

		let Some(hit) = spatial.cast_ray(ray.origin, ray.direction, f32::INFINITY, false, &GameLayer::Ground.to_filter()) else {
			disable();
			return Ok(());
		};

		if matches!(vis, Visibility::Hidden) {
			cmds.entity(state.rts_cursor).insert(Visibility::Inherited);
		}
		cursor.active = true;
		
		cursor_xform.translation = ray.get_point(hit.distance);
		cursor_xform.rotate_y(time.delta_secs() * std::f32::consts::PI);

		cmds.entity(state.rts_cursor).insert(cursor_xform);

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct RtsPlayerInput;

#[derive(InputAction)]
#[action_output(bool)]
pub struct PlaceUnit;
impl PlaceUnit {
	#[tracing::instrument(skip_all)]
	fn on_place(input: On<Complete<PlaceUnit>>, mut cmds: Commands, state: Res<super::RtsState>, xforms: Query<(&Cursor, &GlobalTransform)>) {
		let Some(unit) = state.selected_unit else { return };

		let (cursor, xform) = xforms.get(input.context).unwrap();
		if !cursor.active { return; }
		cmds.trigger(super::units::SpawnUnit::new(unit, xform.compute_transform()));
	}
}
