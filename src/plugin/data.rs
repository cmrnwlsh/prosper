use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.init_state::<LoadState>()
        .add_plugins(TomlAssetPlugin::<Data>::new(&["embedded://data.toml"]))
        .add_systems(Startup, load)
        .add_systems(Update, poll.run_if(in_state(LoadState::Loading)));
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DataHandle(asset_server.load("embedded://data.toml")));
}

fn poll(
    mut state: ResMut<NextState<LoadState>>,
    handle: Res<DataHandle>,
    asset: Res<Assets<Data>>,
) {
    if let Some(asset) = asset.get(&handle.0) {
        state.set(LoadState::Loaded);
        info!("\nitems: {:#?}", asset.items.len(),);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource)]
pub struct DataHandle(pub Handle<Data>);

#[derive(Asset, TypePath, Serialize, Deserialize, Debug)]
pub struct Data {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    #[serde(alias = "sword")]
    Sword,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub kind: ItemType,
}
