use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::Deserialize;

static APPAREL_PATH: &str = "embedded://items/apparel.toml";

pub fn loader(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<Apparel>::new(&[APPAREL_PATH]))
        .init_state::<LoadState>()
        .add_systems(
            Startup,
            |mut commands: Commands, asset_server: Res<AssetServer>| {
                commands.insert_resource(ApparelHandle(asset_server.load(APPAREL_PATH)));
            },
        )
        .add_systems(
            Update,
            (|handle: Res<ApparelHandle>,
              mut apparel: ResMut<Assets<Apparel>>,
              mut state: ResMut<NextState<LoadState>>| {
                if let Some(data) = apparel.get(handle.0.id()) {
                    info!("{:#?}", data);
                    state.set(LoadState::Loaded);
                    apparel.remove(handle.0.id());
                    info!("{:#?}", apparel.get(handle.0.id()));
                }
            })
            .run_if(in_state(LoadState::Loading)),
        );
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
pub struct Apparel(HashMap<String, WearableItem>);

#[derive(Resource)]
struct ApparelHandle(Handle<Apparel>);

#[derive(Deserialize, TypePath, Debug)]
pub struct WearableItem {
    material: String,
}
