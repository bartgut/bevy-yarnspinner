use bevy::app::{App, Plugin};
use bevy::asset::AssetApp;
use crate::asset::asset::{YarnSpinnerDialog, YarnSpinnerDialogLoader};

pub struct YarnSpinnerPlugin;

impl Plugin for YarnSpinnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<YarnSpinnerDialog>()
            .init_asset_loader::<YarnSpinnerDialogLoader>();
    }
}