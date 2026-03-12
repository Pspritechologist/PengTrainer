// Modified from the below source.

/* 
MIT License

Copyright (c) 2024 Vixen and others

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#![doc = r##"
Simple, metrically correct, PBR compatible and randomly painted mesh for better differentiation of prototype objects.

# Example
In pure Bevy probably you will create a prototype floor like that:
```rust
commands.spawn(MaterialMeshBundle {
	mesh: meshes.add(Cuboid::new(50.0, 2.0, 50.0)),
	material: materials.add(Color::RED.into()),
	..default()
});
```
a solid red or any other color which mixing in eyes. Scene with colors like that it will quickly become unreadable, what you can see on the screenshot below:
![Misleading textures](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/prototype_material/misleading_textures.webp)

But with tool from this create you can archive better results just by change few chars:
```rust
commands.spawn((
	Mesh3d(meshes.add(Cuboid::new(50.0, 2.0, 50.0))),
	PrototypeMaterial::new("floor"),
	Transform::default(),
));
```

Previous red color changed to string, why? Because in this case you can simple describe what you want to add here in future like `player hat` or whatever you want. Color is random generated based on this string, which means you will get the same color for every next program run.
And this will be the result of this small changes:
![Prototype material](https://raw.githubusercontent.com/Vixenka/bevy_dev/master/images/prototype_material/showcase.webp)
"##]

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
use bevy::{
	asset::uuid_handle,
	prelude::*,
	render::render_resource::AsBindGroup,
	shader::ShaderRef,
};
use random_color::{options::Luminosity, RandomColor};

const SHADER_HANDLE: Handle<Shader> = uuid_handle!("0ced3da7-55d3-43be-9e04-5637b0e9ceef");

static INITIALIZED: std::sync::Once = std::sync::Once::new();

/// Plugin for [`crate::prototype_material`] feature. Attachts resources and initialization system.
/// # Remarks
/// This plugin is necessary to use [`crate::prototype_material`] feature. It is added to [`App`] by [`crate::DevPlugins`].
pub struct PrototypeMaterialPlugin;

impl Plugin for PrototypeMaterialPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(MaterialPlugin::<PrototypeMaterialAsset>::default())
			.add_systems(PostUpdate, initialization);
	}
}

/// Component which includes [`PrototypeMaterialAsset`] to [`Entity`] in the next [`PostUpdate`].
#[derive(Component, Debug, Clone, Copy)]
pub struct PrototypeMaterial {
	color: Color,
}

impl PrototypeMaterial {
	/// Creates a prototype material with procedural color.
	/// # Arguments
	/// * `color_src` - A Source that a color can be derived from. Examples include an actual color, or a string that describes the feature that this prototype material is for to generate a consistant procedural.
	pub fn new(color_src: impl ColorSource) -> Self {
		Self {
			color: color_src.color(),
		}
	}
}

/// A [`Material`] that uses a [`PrototypeMaterialAsset`] shader.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct PrototypeMaterialAsset {
	#[uniform(0)]
	pub color: LinearRgba,
	#[texture(1)]
	#[sampler(2)]
	pub base_texture: Handle<Image>,
}

pub trait ColorSource {
	fn color(self) -> Color;
}

impl ColorSource for Color {
	fn color(self) -> Color { self }
}

impl ColorSource for &'_ str {
	fn color(self) -> Color {
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		let hash = hasher.finish();

		let rgb = RandomColor::new()
			.luminosity(Luminosity::Bright)
			.seed(hash)
			.to_rgb_array();
		
		Color::srgb_u8(rgb[0], rgb[1], rgb[2])
	}
}

impl PrototypeMaterialAsset {
	pub fn new(
		color_src: impl ColorSource,
		asset_server: &AssetServer,
	) -> Self {
		let base_texture = asset_server.load("textures/prototype.png");

		Self {
			color: color_src.color().to_linear(),
			base_texture,
		}
	}
}

impl Material for PrototypeMaterialAsset {
	fn vertex_shader() -> ShaderRef {
		SHADER_HANDLE.into()
	}

	fn fragment_shader() -> ShaderRef {
		SHADER_HANDLE.into()
	}
}

fn initialization(
	mut commands: Commands,
	entities: Query<(Entity, &PrototypeMaterial), Changed<PrototypeMaterial>>,
	asset_server: Res<AssetServer>,
	mut shaders: ResMut<Assets<Shader>>,
	mut materials: ResMut<Assets<PrototypeMaterialAsset>>,
) {
	if entities.is_empty() {
		return;
	}

	INITIALIZED.call_once(|| {
		shaders.insert(&SHADER_HANDLE, Shader::from_wgsl(
			include_str!("prototype_material.wgsl"),
			std::path::Path::new(file!()).with_file_name("prototype_material.wgsl").to_string_lossy(),
		)).unwrap();
	});

	for (entity, material) in entities {
		commands.entity(entity).insert(MeshMaterial3d(materials.add(PrototypeMaterialAsset::new(
			material.color,
			&asset_server,
		))));
	}
}
