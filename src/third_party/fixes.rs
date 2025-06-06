use bevy::prelude::*;
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;
use bevy_mod_skinned_aabb::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((FixPointerUnlockPlugin, SkinnedAabbPlugin));
}
