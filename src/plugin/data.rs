use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::msgpack::MsgPackAssetPlugin;
use serde::{Deserialize, Serialize};

pub fn data(app: &mut App) {
    app.init_state::<LoadState>()
        .add_plugins(MsgPackAssetPlugin::<Data>::new(&["embedded://data.mpk"]))
        .add_systems(Startup, load)
        .add_systems(Update, poll.run_if(in_state(LoadState::Loading)));
}

fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DataAsset(asset_server.load("embedded://data.mpk")));
}

fn poll(mut state: ResMut<NextState<LoadState>>, handle: Res<DataAsset>, asset: Res<Assets<Data>>) {
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
    #[serde(alias = "cn")]
    pub category_name: String,
    #[serde(alias = "n")]
    pub name: String,
    #[serde(alias = "w")]
    pub weight: f64,
    #[serde(alias = "v")]
    pub volume: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    #[serde(alias = "i")]
    pub inputs: HashMap<String, u32>,
    #[serde(alias = "o")]
    pub outputs: HashMap<String, u32>,
    #[serde(alias = "t")]
    pub time_ms: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    #[serde(alias = "n")]
    pub name: String,
    #[serde(alias = "ex")]
    pub expertise: Option<String>,
    #[serde(alias = "p")]
    pub pioneers: u32,
    #[serde(alias = "se")]
    pub settlers: u32,
    #[serde(alias = "t")]
    pub technicians: u32,
    #[serde(alias = "en")]
    pub engineers: u32,
    #[serde(alias = "sc")]
    pub scientists: u32,
    #[serde(alias = "ac")]
    pub area_cost: u32,
    #[serde(alias = "c")]
    pub costs: HashMap<String, u32>,
}

#[derive(Resource)]
pub struct DataAsset(Handle<Data>);
