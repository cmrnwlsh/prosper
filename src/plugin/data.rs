use bevy::{app::PluginGroupBuilder, prelude::*};
use buildings::BuildingsPlugin;
use items::ItemsPlugin;
use recipes::RecipesPlugin;

pub struct DataPlugins;
impl PluginGroup for DataPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ItemsPlugin)
            .add(BuildingsPlugin)
            .add(RecipesPlugin)
    }
}

mod items {
    use bevy::prelude::*;
    use bevy_common_assets::json::JsonAssetPlugin;
    use serde::{Deserialize, Serialize};

    pub struct ItemsPlugin;
    impl Plugin for ItemsPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoadState>()
                .add_plugins(JsonAssetPlugin::<Items>::new(&["embedded://items.json"]))
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

    #[derive(Deserialize, Asset, TypePath, Debug)]
    pub struct Items(Vec<Item>);

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Item {
        #[serde(rename = "CategoryName")]
        pub category_name: Option<String>,
        #[serde(rename = "Name", alias = "CommodityName")]
        pub name: String,
        #[serde(rename = "Ticker", alias = "CommodityTicker")]
        pub ticker: String,
        #[serde(rename = "Weight")]
        pub weight: f64,
        #[serde(rename = "Volume")]
        pub volume: f64,
        #[serde(rename = "Amount")]
        pub amount: Option<i64>,
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
        if let Some(items) = asset.get(&handle.0) {
            state.set(LoadState::Loaded);
            //info!("{}:\n{:#?}", items.0.len(), items);
        }
    }
}

mod buildings {
    use super::{items::Item, recipes::Recipe};
    use bevy::prelude::*;
    use bevy_common_assets::json::JsonAssetPlugin;
    use serde::{Deserialize, Serialize};

    pub struct BuildingsPlugin;
    impl Plugin for BuildingsPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoadState>()
                .add_plugins(JsonAssetPlugin::<Buildings>::new(&[
                    "embedded://buildings.json",
                ]))
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

    #[derive(Deserialize, Asset, TypePath, Debug)]
    pub struct Buildings(Vec<Building>);

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Building {
        #[serde(rename = "BuildingCosts")]
        pub building_costs: Vec<Item>,
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
            //info!("{}:\n{:#?}", buildings.0.len(), buildings)
        }
    }
}

mod recipes {
    use bevy::{
        asset::{Asset, Handle},
        prelude::*,
        reflect::TypePath,
    };
    use bevy_common_assets::json::JsonAssetPlugin;
    use serde::{Deserialize, Serialize};

    pub struct RecipesPlugin;
    impl Plugin for RecipesPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoadState>()
                .add_plugins(JsonAssetPlugin::<Recipes>::new(&[
                    "embedded://recipes.json",
                ]))
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

    #[derive(Deserialize, Asset, TypePath, Debug)]
    pub struct Recipes(Vec<Recipe>);

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Recipe {
        #[serde(rename = "BuildingTicker")]
        pub building_ticker: Option<String>,
        #[serde(rename = "RecipeName")]
        pub recipe_name: String,
        #[serde(rename = "StandardRecipeName")]
        pub standard_recipe_name: String,
        #[serde(rename = "Inputs")]
        pub inputs: Vec<Commodity>,
        #[serde(rename = "Outputs")]
        pub outputs: Vec<Commodity>,
        #[serde(rename = "TimeMs", alias = "DurationMs")]
        pub time_ms: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Commodity {
        #[serde(rename = "Ticker", alias = "CommodityTicker")]
        pub ticker: String,
        #[serde(rename = "Amount")]
        pub amount: i64,
    }

    #[derive(Resource)]
    pub struct RecipesAsset(Handle<Recipes>);

    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(RecipesAsset(asset_server.load("embedded://recipes.json")));
    }

    pub fn wait(
        mut state: ResMut<NextState<LoadState>>,
        handle: Res<RecipesAsset>,
        asset: Res<Assets<Recipes>>,
    ) {
        if let Some(recipes) = asset.get(&handle.0) {
            state.set(LoadState::Loaded);
            info!("{}:\n{:#?}", recipes.0.len(), recipes)
        }
    }
}
