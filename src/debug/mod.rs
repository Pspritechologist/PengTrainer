use bevy::prelude::*;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};
use std::time::Duration;

mod prototype_mat;

pub use prototype_mat::*;

pub fn fps_overlay() -> FpsOverlayPlugin {
	FpsOverlayPlugin {
		config: FpsOverlayConfig {
			text_config: TextFont { font_size: 16.0, ..default() },
			refresh_interval: Duration::from_millis(100),
			enabled: true,
			frame_time_graph_config: FrameTimeGraphConfig {
				enabled: true,
				min_fps: 30.0,
				target_fps: 144.0,
			},
			..Default::default()
		},
	}
}
