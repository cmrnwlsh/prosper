use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::{Deserialize, Serialize};

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadState>()
            .add_plugins(TomlAssetPlugin::<Data>::new(&["embedded://data.toml"]))
            .add_systems(Startup, load)
            .add_systems(Update, wait.run_if(in_state(LoadState::Loading)));
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LoadState {
    #[default]
    Loading,
    Loaded,
}

pub type Items = HashMap<String, Item>;
pub type Recipes = HashMap<String, Recipe>;
pub type Buildings = HashMap<String, Building>;

#[derive(Asset, TypePath)]
pub struct Data {
    items: Items,
    recipes: Recipes,
    buildings: Buildings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub category_name: String,
    pub name: String,
    pub ticker: String,
    pub weight: f64,
    pub volume: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub building_ticker: String,
    pub recipe_name: String,
    pub standard_recipe_name: String,
}

#[derive(Resource)]
pub struct DataAsset(Handle<Data>);

pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(DataAsset(asset_server.load("embedded://data.toml")));
}

pub fn wait(
    mut state: ResMut<NextState<LoadState>>,
    handle: Res<DataAsset>,
    asset: Res<Assets<Data>>,
) {
    if let Some(asset) = asset.get(&handle.0) {
        state.set(LoadState::Loaded);
        info!("{}", asset.items.len());
    }
}
