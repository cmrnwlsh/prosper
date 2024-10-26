use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::{Deserialize, Serialize};

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadState>()
            .add_plugins(TomlAssetPlugin::<Data>::new(&["embedded://data.toml"]))
            .add_systems(Startup, load)
            .add_systems(Update, poll.run_if(in_state(LoadState::Loading)));
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
    items: HashMap<String, Item>,
    recipes: Vec<Recipe>,
    buildings: Vec<Building>,
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
    pub building_ticker: String,
    pub recipe_name: String,
    pub standard_recipe_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Building {
    pub name: String,
    pub ticker: String,
    pub expertise: Option<String>,
    pub pioneers: u16,
    pub settlers: u16,
    pub technicians: u16,
    pub engineers: u16,
    pub scientists: u16,
    pub area_cost: u16,
    pub recipes: Vec<String>,
    pub costs: Vec<BuildingCost>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuildingCost {
    pub amount: u16,
    pub ticker: String,
}

#[derive(Resource)]
pub struct DataAsset(Handle<Data>);

pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DataAsset(asset_server.load("embedded://data.toml")));
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
            asset.recipes.len(),
            asset.buildings.len()
        );
    }
}
