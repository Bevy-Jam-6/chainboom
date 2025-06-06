//! Assets for the player.

use bevy::prelude::*;
use bevy_shuffle_bag::ShuffleBag;

use crate::{
    asset_tracking::LoadResource, third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

use super::Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct PlayerAssets {
    #[dependency]
    pub(crate) _model: Handle<Scene>,
    #[dependency]
    pub(crate) throw_sound: Handle<AudioSource>,
    #[dependency]
    pub(crate) steps: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) jump_grunts: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) land_sounds: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) jump_start_sounds: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) shooting_sounds: ShuffleBag<Handle<AudioSource>>,
    #[dependency]
    pub(crate) reload_sound: Handle<AudioSource>,
    #[dependency]
    pub(crate) hidden_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) idle_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) walk_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) shoot_animation: Handle<AnimationClip>,
    #[dependency]
    pub(crate) hurt_sounds: ShuffleBag<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        let rng = &mut rand::thread_rng();
        Self {
            _model: assets.load(Player::scene_path()),
            throw_sound: assets.load("audio/sound_effects/throw.ogg"),
            steps: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_01.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_02.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_03.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_04.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_05.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_06.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_07.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_08.ogg"),
                    assets.load("audio/sound_effects/step/Footsteps_Rock_Walk_09.ogg"),
                ],
                rng,
            )
            .unwrap(),
            jump_grunts: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_1.ogg"),
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_2.ogg"),
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_3.ogg"),
                    assets.load("audio/sound_effects/jump_grunt/jump_grunt_4.ogg"),
                ],
                rng,
            )
            .unwrap(),
            land_sounds: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_01.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_02.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_03.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_04.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_05.ogg"),
                    assets.load("audio/sound_effects/land/Footsteps_Rock_Jump_Land_06.ogg"),
                ],
                rng,
            )
            .unwrap(),
            jump_start_sounds: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_01.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_02.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_03.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_04.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_05.ogg"),
                    assets.load("audio/sound_effects/jump_start/Footsteps_Rock_Jump_Start_06.ogg"),
                ],
                rng,
            )
            .unwrap(),
            shooting_sounds: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/shoot/Shotgun_Shot-001.ogg"),
                    assets.load("audio/sound_effects/shoot/Shotgun_Shot-002.ogg"),
                    assets.load("audio/sound_effects/shoot/Shotgun_Shot-003.ogg"),
                    assets.load("audio/sound_effects/shoot/Shotgun_Shot-004.ogg"),
                ],
                rng,
            )
            .unwrap(),
            reload_sound: assets.load("audio/sound_effects/shoot/Shotgun_Pump.ogg"),
            hurt_sounds: ShuffleBag::try_new(
                [
                    assets.load("audio/sound_effects/hurt/damage_1_meghan.ogg"),
                    assets.load("audio/sound_effects/hurt/damage_2_meghan.ogg"),
                    assets.load("audio/sound_effects/hurt/damage_4_meghan.ogg"),
                    assets.load("audio/sound_effects/hurt/damage_7_meghan.ogg"),
                ],
                rng,
            )
            .unwrap(),
            hidden_animation: assets.load(Player::animation_path(0)),
            idle_animation: assets.load(Player::animation_path(1)),
            shoot_animation: assets.load(Player::animation_path(2)),
            walk_animation: assets.load(Player::animation_path(3)),
        }
    }
}
