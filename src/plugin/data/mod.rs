mod item;

use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::Deserialize;

static DATA_PATH: &str = "embedded://data.toml";

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<Data>::new(&[DATA_PATH]))
            .init_state::<LoadState>()
            .add_systems(Startup, load_data)
            .add_systems(Update, poll_data.run_if(in_state(LoadState::Loading)));
    }
}

fn load_data(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DataHandle(asset_server.load(DATA_PATH)))
}

fn poll_data(
    handle: Res<DataHandle>,
    mut commands: Commands,
    mut res_data: ResMut<Assets<Data>>,
    mut state: ResMut<NextState<LoadState>>,
) {
    if let Some(data) = res_data.remove(handle.0.id()) {
        info!("{:#?}", data);
        commands.insert_resource(data);
        state.set(LoadState::Loaded);
    }
}

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Deserialize, Resource, Asset, TypePath, Debug)]
pub struct Data {
    apparel: HashMap<String, item::Wearable>,
}

#[derive(Resource)]
struct DataHandle(Handle<Data>);
