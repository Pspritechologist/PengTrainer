use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use tracing::instrument;
use bevy_egui::egui;

use super::units::{UnitData, UnitList};

use crate::debug::PrototypeMaterial;

pub fn plugin(app: &mut App) {
	#[derive(Resource)]
	struct FloaterMesh(Handle<Mesh>);

	let floater = UnitData::new("Floater", |ent| {
		let mesh = ent.get_resource::<FloaterMesh>().map(|m| m.0.clone()).unwrap_or_else(|| {
			let mesh = ent.resource_mut::<Assets<Mesh>>().add(Cuboid::from_length(1.0));
			ent.world_scope(|w| w.insert_resource(FloaterMesh(mesh.clone())));
			mesh
		});
		
		ent.insert((
			Name::new("Cuboid"),
			Collider::cuboid(1.0, 1.0, 1.0),
			Mesh3d(mesh),
			PrototypeMaterial::new("cuboid"),
			// Transform::from_xyz(0., 20., 0.),
			crate::movement::Floater::default(),
			crate::movement::FloatMovement {
				acceleration: 8.0,
				max_speed: 3.2,
				dimeyness: 4.0,
				..Default::default()
			},
		));

		Ok(())
	});

	let mut unit_list = UnitList::default();
	unit_list.extend(std::iter::repeat_n(floater, 87));

	app
		.insert_resource(unit_list)
		.insert_resource(UnitBrowserState(true))
		.add_systems(bevy_egui::EguiPrimaryContextPass, render_unit_browser)
	;
}

#[derive(Debug, Clone, Copy, Resource, Reflect)]
struct UnitBrowserState(bool);

#[instrument(skip_all)]
fn render_unit_browser(
	mut cmds: Commands,
	open: Res<UnitBrowserState>,
	units: Res<UnitList>,
	mut ctxs: bevy_egui::EguiContexts,
) -> Result {
	let ctx = ctxs.ctx_mut()?;
	
	ctx.style_mut(|style| {
		let color = style.visuals.panel_fill;
		let color = egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 128);
		style.visuals.panel_fill = color;
	});

	egui::TopBottomPanel::bottom("UnitBrowser").resizable(true).default_height(120.0).show_animated(ctx, open.0, |ui| {
		egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
			ui.vertical_centered_justified(|ui| {
				ui.horizontal_wrapped(|ui| {
					for (id, unit) in units.units() {
						ui.horizontal(|ui| {
							if ui.button(unit.name()).clicked() {
								cmds.trigger(super::SelectUnit(Some(id)));
							}
						});
					}
				});
			});
		});
	});

	Ok(())
}
