use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_enhanced_input::prelude::*;
use tracing::instrument;

use crate::scratch::inventory as inv;

pub fn init(app: &mut App) {
	app
		.add_observer(OpenInv::on_open_inv)
		.add_systems(EguiPrimaryContextPass, display_list_invs)
	;
}

/// This Entity has an openable Inventory presented as a simple list of items.
#[derive(Debug, Clone, Copy, Component, Reflect)]
#[require(inv::Inventory)]
pub struct ListInv {
	
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct OpenInv;
impl OpenInv {
	#[instrument(skip_all)]
	fn on_open_inv(input: On<Start<Self>>, mut cmds: Commands, query: Populated<Option<&HasListInvOpen>, With<ListInv>>) -> Result {
		// let Ok(has_list_inv) = query.get(input.context) else { return };
		let list_inv_open = query.get(input.context)?;
		if list_inv_open.is_some() {
			cmds.entity(input.context).remove::<HasListInvOpen>();
		} else {
			cmds.entity(input.context).insert(HasListInvOpen::new(input.context));
		}

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, Component, Reflect)]
pub struct HasListInvOpen {
	inventory: Entity,
}
impl HasListInvOpen {
	fn new(inventory: Entity) -> Self {
		Self { inventory }
	}
}

fn display_list_invs(
	mut cmds: Commands,
	inventories: Query<(NameOrEntity, &mut ListInv, &inv::Inventory), With<HasListInvOpen>>,
	items: Query<(NameOrEntity, &inv::Item)>,
	mut ctxs: EguiContexts,
) -> Result {
	let ctx = ctxs.ctx_mut()?;

	for (inventory, list_inv, inv_comp) in inventories {
		// let ctx = ctxs.ctx_for_entity_mut(inventory)?;

		egui::Window::new("invy").id(egui::Id::new(inventory.entity)).vscroll(true).show(ctx, |ui| {
			for (ent, item) in items.iter_many(inv_comp.collection()) {
				ui.horizontal(|ui| {
					ui.label(ent.to_string());
					if ui.small_button("Eject").clicked() {
						info!("Ejected {ent}");
						cmds.entity(item.represented_entity()).queue(inv::RemoveFromInventoryCmd);
					}
				});
			}

			if inv_comp.collection().is_empty() {
				ui.label("This inventory is empty :(");
			}
		});
	}

	Ok(())
}
