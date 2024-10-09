use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use buildings::Buildings;
use items::Items;
use serde::{Deserialize, Serialize};

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<Items>::new(&["embedded://items.json"]))
            .add_plugins(JsonAssetPlugin::<Buildings>::new(&[
                "embedded://buildings.json",
            ]))
            .init_state::<items::LoadState>()
            .init_state::<buildings::LoadState>()
            .add_systems(Startup, (items::load, buildings::load))
            .add_systems(
                Update,
                (
                    items::wait.run_if(in_state(items::LoadState::Loading)),
                    buildings::wait.run_if(in_state(buildings::LoadState::Loading)),
                ),
            );
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commodity {
    #[serde(rename = "CommodityName")]
    pub commodity_name: String,
    #[serde(rename = "CommodityTicker")]
    pub commodity_ticker: String,
    #[serde(rename = "Weight")]
    pub weight: f64,
    #[serde(rename = "Volume")]
    pub volume: f64,
    #[serde(rename = "Amount")]
    pub amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    #[serde(rename = "Inputs")]
    pub inputs: Vec<Commodity>,
    #[serde(rename = "Outputs")]
    pub outputs: Vec<Commodity>,
    #[serde(rename = "DurationMs")]
    pub duration_ms: i64,
    #[serde(rename = "RecipeName")]
    pub recipe_name: String,
    #[serde(rename = "StandardRecipeName")]
    pub standard_recipe_name: String,
}

mod items {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
    pub enum LoadState {
        #[default]
        Loading,
        Loaded,
    }

    #[derive(Deserialize, Asset, TypePath, Debug)]
    pub struct Items(Vec<Item>);

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Item {
        #[serde(rename = "CategoryName")]
        pub category_name: String,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "Ticker")]
        pub ticker: String,
        #[serde(rename = "Weight")]
        pub weight: f64,
        #[serde(rename = "Volume")]
        pub volume: f64,
    }

    #[derive(Resource)]
    pub struct ItemsAsset(Handle<Items>);

    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(ItemsAsset(asset_server.load("embedded://items.json")));
    }

    pub fn wait(
        mut state: ResMut<NextState<LoadState>>,
        handle: Res<ItemsAsset>,
        asset: Res<Assets<Items>>,
    ) {
        if asset.get(&handle.0).is_some() {
            state.set(LoadState::Loaded)
        }
    }
}

mod buildings {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    use super::{Commodity, Recipe};

    #[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
    pub enum LoadState {
        #[default]
        Loading,
        Loaded,
    }

    #[derive(serde::Deserialize, Asset, TypePath, Debug)]
    pub struct Buildings(Vec<Building>);

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Building {
        #[serde(rename = "BuildingCosts")]
        pub building_costs: Vec<Commodity>,
        #[serde(rename = "Recipes")]
        pub recipes: Vec<Recipe>,
        #[serde(rename = "Name")]
        pub name: String,
        #[serde(rename = "Ticker")]
        pub ticker: String,
        #[serde(rename = "Expertise")]
        pub expertise: Option<String>,
        #[serde(rename = "Pioneers")]
        pub pioneers: i64,
        #[serde(rename = "Settlers")]
        pub settlers: i64,
        #[serde(rename = "Technicians")]
        pub technicians: i64,
        #[serde(rename = "Engineers")]
        pub engineers: i64,
        #[serde(rename = "Scientists")]
        pub scientists: i64,
        #[serde(rename = "AreaCost")]
        pub area_cost: i64,
    }

    #[derive(Resource)]
    pub struct BuildingsAsset(Handle<Buildings>);

    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(BuildingsAsset(
            asset_server.load("embedded://buildings.json"),
        ));
    }

    pub fn wait(
        mut state: ResMut<NextState<LoadState>>,
        handle: Res<BuildingsAsset>,
        asset: Res<Assets<Buildings>>,
    ) {
        if let Some(buildings) = asset.get(&handle.0) {
            state.set(LoadState::Loaded);
            info!("{}:\n{:#?}", buildings.0.len(), buildings)
        }
    }
}
