use bevy::{app::{App, PostUpdate}, ecs::schedule::IntoScheduleConfigs, transform::TransformSystems};

pub use transform_prop::*;

mod transform_prop;

pub fn plugin(app: &mut App) {
	app.add_systems(PostUpdate, transform_prop::update.after(TransformSystems::Propagate));
}

pub trait WithAppended {
	type Item;
	fn with_appended<T: Into<Self::Item>>(self, item: T) -> Self;
	fn with_appended_iter(mut self, items: impl IntoIterator<Item = Self::Item>) -> Self where Self: Sized {
		for item in items {
			self = self.with_appended(item);
		}
		self
	}
	fn with_appended_slice(self, items: &[Self::Item]) -> Self where Self: Sized, Self::Item: Copy {
		self.with_appended_iter(items.iter().copied())
	}
}

impl<Item> WithAppended for Vec<Item> {
	type Item = Item;
	fn with_appended<T: Into<Self::Item>>(mut self, item: T) -> Self {
		self.push(item.into());
		self
	}
	fn with_appended_iter(mut self, items: impl IntoIterator<Item = Self::Item>) -> Self where Self: Sized {
		self.extend(items);
		self
	}
	fn with_appended_slice(mut self, items: &[Self::Item]) -> Self where Self: Sized, Self::Item: Copy {
		self.extend(items);
		self
	}
}
