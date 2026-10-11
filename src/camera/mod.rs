use crate::markers;
use crate::screens::Screen;
use crate::shared::Config;
#[cfg(feature = "third_person")]
use crate::third_party::ThirdPersonCamera;
use bevy::prelude::*;
#[cfg(not(feature = "third_person"))]
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
#[cfg(feature = "native")]
use bevy::{anti_alias::taa::TemporalAntiAliasing, pbr::ScreenSpaceAmbientOcclusion};
use bevy::{
    camera::{Exposure, Hdr, ScalingMode},
    core_pipeline::tonemapping::Tonemapping,
    light::ShadowFilteringMethod,
    post_process::bloom::Bloom,
};

pub mod diorama;
// mod gamepad_cursor;
mod hdr;
#[cfg(feature = "third_person")]
mod third_person;
#[cfg(feature = "top_down")]
mod top_down;

pub use diorama::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((hdr::plugin, diorama::plugin))
        // app.add_plugins((hdr::plugin, gamepad_cursor::plugin))
        .add_systems(Startup, spawn_camera)
        .add_observer(on_toggle_cam_cursor);

    #[cfg(feature = "third_person")]
    app.add_plugins(third_person::plugin);
    #[cfg(feature = "top_down")]
    app.add_plugins(top_down::plugin);
}

markers!(SceneCamera);

#[derive(EntityEvent)]
pub struct ToggleCamCursor(pub Entity);

pub fn spawn_camera(mut commands: Commands) {
    let config = DioramaCameraConfig::default();
    let initial_yaw = config.target_yaw_radians();
    let initial_offset = diorama::compute_camera_offset(config.pitch_radians, initial_yaw, config.distance);
    let initial_target = Vec3::new(3.5, 0.0, 3.5); // Center of Cradle Meadows

    commands.spawn((
        SceneCamera,
        DioramaCamera,
        IsDefaultUiCamera,
        Camera::default(),
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config.vertical_size,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(initial_target + initial_offset).looking_at(initial_target, Vec3::Y),
        (
            Exposure::SUNLIGHT,
            Tonemapping::TonyMcMapface,
            Bloom::NATURAL,
            Hdr,
        ),
        // performance critical
        (
            Msaa::Off,
            #[cfg(feature = "native")] // breaks wasm
            TemporalAntiAliasing::default(),
            ShadowFilteringMethod::Temporal,
            #[cfg(feature = "native")] // See https://github.com/bPluginevyengine/bevy/issues/20459
            ScreenSpaceAmbientOcclusion::default(),
        ),
    ));
}

// TODO: make it work in a local split screen
fn on_toggle_cam_cursor(
    _: On<ToggleCamCursor>,
    #[cfg(feature = "third_person")] mut cam: Query<&mut ThirdPersonCamera>,
    #[cfg(not(feature = "third_person"))] mut window_q: Query<
        &mut CursorOptions,
        With<PrimaryWindow>,
    >,
) {
    #[cfg(feature = "third_person")]
    if let Ok(mut cam) = cam.single_mut() {
        cam.cursor_lock_active = !cam.cursor_lock_active;
        return;
    };

    #[cfg(not(feature = "third_person"))]
    if let Ok(mut cursor_options) = window_q.single_mut() {
        cursor_options.visible = !cursor_options.visible;
        debug!("cursor will be visible: {}", cursor_options.visible);
        if cursor_options.visible {
            cursor_options.grab_mode = CursorGrabMode::None;
        } else {
            cursor_options.grab_mode = CursorGrabMode::Locked;
        }
    }
}
