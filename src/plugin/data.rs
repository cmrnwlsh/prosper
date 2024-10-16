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
    use bevy_common_assets::toml::TomlAssetPlugin;
    use serde::{Deserialize, Serialize};

    pub struct ItemsPlugin;
    impl Plugin for ItemsPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoadState>()
                .add_plugins(TomlAssetPlugin::<Items>::new(&["embedded://items.toml"]))
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
    pub struct Items {
        items: Vec<Item>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Item {
        pub category_name: Option<String>,
        #[serde(alias = "commodity_name")]
        pub name: String,
        #[serde(alias = "commodity_ticker")]
        pub ticker: String,
        pub weight: f64,
        pub volume: f64,
        pub amount: Option<i64>,
    }

    #[derive(Resource)]
    pub struct ItemsAsset(Handle<Items>);

    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(ItemsAsset(asset_server.load("embedded://items.toml")));
    }

    pub fn wait(
        mut state: ResMut<NextState<LoadState>>,
        handle: Res<ItemsAsset>,
        asset: Res<Assets<Items>>,
    ) {
        if let Some(asset) = asset.get(&handle.0) {
            state.set(LoadState::Loaded);
            info!("{}", asset.items.len());
        }
    }
}

mod buildings {
    use super::{items::Item, recipes::Recipe};
    use bevy::prelude::*;
    use bevy_common_assets::toml::TomlAssetPlugin;
    use serde::{Deserialize, Serialize};

    pub struct BuildingsPlugin;
    impl Plugin for BuildingsPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoadState>()
                .add_plugins(TomlAssetPlugin::<Buildings>::new(&[
                    "embedded://buildings.toml",
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
    pub struct Buildings {
        buildings: Vec<Building>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Building {
        pub building_costs: Vec<Item>,
        pub recipes: Vec<Recipe>,
        pub name: String,
        pub ticker: String,
        pub expertise: Option<String>,
        pub pioneers: i64,
        pub settlers: i64,
        pub technicians: i64,
        pub engineers: i64,
        pub scientists: i64,
        pub area_cost: i64,
    }

    #[derive(Resource)]
    pub struct BuildingsAsset(Handle<Buildings>);

    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(BuildingsAsset(
            asset_server.load("embedded://buildings.toml"),
        ));
    }

    pub fn wait(
        mut state: ResMut<NextState<LoadState>>,
        handle: Res<BuildingsAsset>,
        asset: Res<Assets<Buildings>>,
    ) {
        if let Some(asset) = asset.get(&handle.0) {
            state.set(LoadState::Loaded);
            info!("{}", asset.buildings.len())
        }
    }
}

mod recipes {
    use bevy::{
        asset::{Asset, Handle},
        prelude::*,
        reflect::TypePath,
    };
    use bevy_common_assets::toml::TomlAssetPlugin;
    use serde::{Deserialize, Serialize};

    pub struct RecipesPlugin;
    impl Plugin for RecipesPlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<LoadState>()
                .add_plugins(TomlAssetPlugin::<Recipes>::new(&[
                    "embedded://recipes.toml",
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
    pub struct Recipes {
        recipes: Vec<Recipe>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Recipe {
        pub building_ticker: Option<String>,
        pub recipe_name: String,
        pub standard_recipe_name: String,
        pub inputs: Vec<Commodity>,
        pub outputs: Vec<Commodity>,
        #[serde(alias = "duration_ms")]
        pub time_ms: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Commodity {
        #[serde(alias = "commodity_ticker")]
        pub ticker: String,
        pub amount: i64,
    }

    #[derive(Resource)]
    pub struct RecipesAsset(Handle<Recipes>);

    pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.insert_resource(RecipesAsset(asset_server.load("embedded://recipes.toml")));
    }

    pub fn wait(
        mut state: ResMut<NextState<LoadState>>,
        handle: Res<RecipesAsset>,
        asset: Res<Assets<Recipes>>,
    ) {
        if let Some(asset) = asset.get(&handle.0) {
            state.set(LoadState::Loaded);
            info!("{}", asset.recipes.len())
        }
    }
}
