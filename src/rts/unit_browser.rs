use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use avian3d::prelude::*;
use tracing::instrument;
use bevy_egui::egui;

use crate::debug::PrototypeMaterial;

pub fn plugin(app: &mut App) {
	let floater = UnitData {
		name: "Floater".to_string(),
		icon: None,
		spawner: |world| {
			let mesh = world.resource_mut::<Assets<Mesh>>().add(Cuboid::from_length(1.0));
			world.spawn((
				Name::new("Cuboid"),
				Collider::cuboid(1.0, 1.0, 1.0),
				Mesh3d(mesh),
				PrototypeMaterial::new("cuboid"),
				Transform::from_xyz(0., 20., 0.),
				crate::movement::Floater::default(),
				crate::movement::FloatMovement {
					acceleration: 8.0,
					max_speed: 3.2,
					dimeyness: 4.0,
					..Default::default()
				},
			));
		},
	};

	app
		.insert_resource(UnitList(vec![floater.clone(), floater.clone(), floater.clone(), floater.clone(), floater.clone(), floater.clone(), floater.clone(), floater.clone(), floater]))
		.insert_resource(UnitBrowserState(true))
		.add_systems(bevy_egui::EguiPrimaryContextPass, render_unit_browser)
	;
}

#[derive(Debug, Clone, Default, Resource)]
struct UnitList(Vec<UnitData>);

#[derive(Debug, Clone, Reflect)]
#[reflect(from_reflect = false)]
struct UnitData {
	name: String,
	icon: Option<u8>,
	#[reflect(ignore)]
	spawner: fn(&mut World),
}
impl Default for UnitData {
	fn default() -> Self {
		Self { icon: default(), name: default(), spawner: |_| () }
	}
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
					for unit in &units.0 {
						ui.horizontal(|ui| {
							if let Some(icon) = unit.icon {
								ui.label(format!("Icon {icon}"));
							}
							if ui.button(&unit.name).clicked() {
								let spawner = unit.spawner;
								cmds.queue(move |world: &mut World| spawner(world));
							}
						});
					}
				});
			});
		});
	});

	Ok(())
}
