use crate::asset_loading::AudioSources;
use crate::audio::{FadeIn, FadeOut};
use crate::camera::ToggleCamCursor;
use crate::scene::FadeCam;
use crate::screens::Screen;
use crate::shared::Settings;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_seedling::{prelude::*, sample_effects};

#[cfg(feature = "dev_native")]
mod dev_tools;
pub mod autoclave;
pub mod building;
pub mod crafting;
pub mod diorama_world;
mod dialogue;
pub mod grid;
pub mod harmonic;
pub mod hud;
pub mod inventory;
pub mod items;
pub mod logistics;
pub mod mining;
mod mood;
pub mod player_interaction;
pub mod polyomino;
pub mod prospecting;
pub mod terrain;

pub use autoclave::*;
pub use building::*;
pub use crafting::*;
pub use diorama_world::*;
pub use grid::*;
pub use harmonic::*;
pub use hud::*;
pub use inventory::*;
pub use items::*;
pub use logistics::*;
pub use mining::*;
pub use mood::*;
pub use player_interaction::*;
pub use polyomino::*;
pub use prospecting::*;
pub use terrain::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        mood::plugin,
        #[cfg(feature = "dev_native")]
        dev_tools::plugin,
        dialogue::plugin,
        terrain::plugin,
        items::plugin,
        inventory::plugin,
        crafting::plugin,
        mining::plugin,
    ))
    .add_plugins((
        polyomino::plugin,
        autoclave::plugin,
        logistics::plugin,
        building::plugin,
        prospecting::plugin,
        harmonic::plugin,
        diorama_world::plugin,
        hud::plugin,
        player_interaction::plugin,
    ))
    .add_observer(pause)
    .add_observer(mute);
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct Pause;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Mute;

#[derive(EntityEvent)]
pub struct TogglePause(pub Entity);
#[derive(EntityEvent)]
pub struct ToggleMute(pub Entity);
#[derive(Event)]
pub struct ToggleDebugUi;

// ================== trigger events on input ========================
fn pause(on: On<Start<Pause>>, mut commands: Commands) {
    commands.entity(on.event_target()).trigger(TogglePause);
}
fn mute(on: On<Start<Mute>>, mut commands: Commands) {
    commands.entity(on.event_target()).trigger(ToggleMute);
}
