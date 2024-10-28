use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::msgpack::MsgPackAssetPlugin;
use serde::{Deserialize, Serialize};

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadState>()
            .add_plugins(MsgPackAssetPlugin::<Data>::new(&["embedded://data.mpk"]))
            .add_systems(Startup, load)
            .add_systems(Update, poll.run_if(in_state(LoadState::Loading)));
    }
}

pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DataAsset(asset_server.load("embedded://data.mpk")));
}

pub fn poll(
    mut state: ResMut<NextState<LoadState>>,
    handle: Res<DataAsset>,
    asset: Res<Assets<Data>>,
) {
    if let Some(asset) = asset.get(&handle.0) {
        state.set(LoadState::Loaded);
        info!(
            "\nitems: {}\nrecipes: {}\nbuildings: {}",
            asset.items.len(),
            asset
                .recipes
                .iter()
                .map(|(_, v)| v.iter().len())
                .sum::<usize>(),
            asset.buildings.len()
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Asset, TypePath, Serialize, Deserialize, Debug)]
pub struct Data {
    pub items: HashMap<String, Item>,
    pub recipes: HashMap<String, Vec<Recipe>>,
    pub buildings: HashMap<String, Building>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub category_name: String,
    pub name: String,
    pub weight: f64,
    pub volume: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub inputs: HashMap<String, u32>,
    pub outputs: HashMap<String, u32>,
    pub time_ms: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub expertise: Option<String>,
    pub pioneers: u32,
    pub settlers: u32,
    pub technicians: u32,
    pub engineers: u32,
    pub scientists: u32,
    pub area_cost: u32,
    pub costs: HashMap<String, u32>,
}

#[derive(Resource)]
pub struct DataAsset(Handle<Data>);
