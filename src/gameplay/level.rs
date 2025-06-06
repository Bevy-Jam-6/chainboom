//! Spawn the main level.

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{asset_tracking::LoadResource, audio::music, gameplay::waves::Waves, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
}

/// A system that spawns the main level.
#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        SceneRoot(level_assets.level.clone()),
        StateScoped(Screen::Gameplay),
        Level,
        children![(Name::new("Level Music"), music(level_assets.music.clone()))],
    ));
    commands.spawn((
        Name::new("Waves"),
        StateScoped(Screen::Gameplay),
        Waves::default(),
    ));
    commands.insert_resource(AmbientLight::NONE);
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
    #[dependency]
    pub(crate) music: Handle<AudioSource>,
    #[dependency]
    pub(crate) env_map_specular: Handle<Image>,
    #[dependency]
    pub(crate) env_map_diffuse: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            level: assets.load("maps/main/main.map#Scene"),
            music: assets.load("audio/music/chainboom.ogg"),
            env_map_specular: assets.load("cubemaps/NightSkyHDRI001_4K-HDR_specular.ktx2"),
            env_map_diffuse: assets.load("cubemaps/NightSkyHDRI001_4K-HDR_diffuse.ktx2"),
        }
    }
}
