use bevy::prelude::*;

/// The game client uses this constant to customize UI, etc.
pub const PROPRIETARY: bool = false;

/// Placeholder for the real MineWars Proprietary Plugin
///
/// In open-source builds, this plugin provides any replacements
/// for anything that the game requires, which, in official/proprietary
/// builds would come from the IyesGames/minewars-proprietary repo.
pub struct MwProprietaryPlugin;

impl Plugin for MwProprietaryPlugin {
    fn build(&self, _app: &mut App) {}
}
