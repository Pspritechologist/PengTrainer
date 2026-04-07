use bevy::{ecs::entity_disabling::Disabled, prelude::*};

pub fn init(app: &mut App) {
	app
		// .register_type::<Inventory>()
		// .register_type::<Capacity>()
		// .register_type::<InInventory>()
		// .register_type::<Item>()
		// .register_type::<IsItem>()
	;
}

/// Queues up creating an Inventory item marker Entity to represent the given `item` Entity, relating them together as well as the
/// marker to the Inventory itself, triggering the [`ItemAdded`] event, and finally disabling the item Entity all in order.
pub fn add_to_inventory(cmds: &mut Commands, inventory: Entity, item: Entity) {
	let item_marker = cmds.spawn_empty().id();

	cmds.entity(item).add_one_related::<Item>(item_marker);
	cmds.entity(inventory).add_one_related::<InInventory>(item_marker);

	cmds.entity(item).insert(Disabled);

	cmds.trigger(ItemAdded { inventory, item });
}

//TODO: Should this take the actual Entity, the marker Entity, or either?
/// Entity Command to 
pub struct RemoveFromInventoryCmd;
impl EntityCommand for RemoveFromInventoryCmd {
	fn apply(self, entity: EntityWorldMut) {
		let item = entity.id();

		let Some(inv_item) = entity.get::<IsItem>().map(|i| i.0) else {
			warn!("Tried to remove ent {} from inventory- Entity was not in inventory", item);
			return
		};

		let world = entity.into_world_mut();

		let inventory = world.entity(inv_item).get::<InInventory>().unwrap().0;

		world.despawn(inv_item);
	
		world.trigger(ItemRemoved { inventory, item });

		world.entity_mut(item).remove::<Disabled>();
	}
}

// Events for when items get added to/removed from an inv

#[derive(Debug, Clone, Copy, Event)]
#[non_exhaustive]
pub struct ItemAdded {
	pub inventory: Entity,
	pub item: Entity,
}

#[derive(Debug, Clone, Copy, Event)]
#[non_exhaustive]
pub struct ItemRemoved {
	pub inventory: Entity,
	pub item: Entity,
}

/// Relation target for the [`InInventory`] Relationship, representing that an Entity is in some way capable of 'storing' other Entities.
/// This is the primary Component for the Inventory system.\
/// This Component provides no default behavior or method of interacting with the inventory.
#[derive(Debug, Clone, Default, Component, Reflect)]
#[reflect(Debug, Clone, Default, Component)]
#[relationship_target(relationship = InInventory, linked_spawn)]
pub struct Inventory {
	contained: Vec<Entity>,
}

/// Relationship representing that an Entity is currently stored in an [`Inventory`].
/// 
/// The Entity this Component belongs to is generally a special item Entity, and only serves as a
/// marker that something is contained in an Inventory. To get the Entity this item Entity refers to,
/// see the [`Item`] Relationship and [`IsItem`] Relation target.
#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Debug, Clone, Component)]
#[relationship(relationship_target = Inventory)]
pub struct InInventory(Entity);

/// Maximum capacity of an [`Inventory`]. This is optional.
#[derive(Debug, Clone, Copy, Default, Component, Reflect, Deref, DerefMut)]
#[reflect(Debug, Clone, Default, Component)]
#[require(Inventory)]
pub struct Capacity(u32);

/// Relationship representing that an Entity is the 'item' belonging to an [`Inventory`], generally serving as a marker that
/// some other Entity is stored in this Inventory. The target for this Relationship is the [`IsItem`] Component.
#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Debug, Clone, Component)]
#[relationship(relationship_target = IsItem)]
pub struct Item(Entity);
impl Item {
	pub fn represented_entity(&self) -> Entity { self.0 }
}

/// Relation target for the [`Item`] Relationship, representing that this Entity has a marker 'Item' Entity stored in an Inventory.
#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Debug, Clone, Component)]
#[relationship_target(relationship = Item)]
pub struct IsItem(Entity);
impl IsItem {
	pub fn inventory_marker_entity(&self) -> Entity { self.0 }
}
