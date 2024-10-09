use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::json::JsonAssetPlugin;

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(JsonAssetPlugin::<Items>::new(&["items.json"]))
            .init_state::<LoadState>()
            .add_systems(Startup, load_items)
            .add_systems(Update, await_items.run_if(in_state(LoadState::Loading)));
    }
}

fn load_items(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = ItemsAsset(asset_server.load("items.json"));
    commands.insert_resource(handle);
}

fn await_items(
    mut state: ResMut<NextState<LoadState>>,
    handle: Res<ItemsAsset>,
    asset: Res<Assets<Items>>,
) {
    if let Some(items) = asset.get(&handle.0) {
        info!("{:#?}", items);
        state.set(LoadState::Loaded)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum LoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct Items(Vec<Item>);

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
struct Item {
    #[serde(alias = "MaterialId")]
    material_id: String,
    #[serde(alias = "CategoryName")]
    category_name: String,
    #[serde(alias = "CategoryId")]
    category_id: String,
    #[serde(alias = "Name")]
    name: String,
    #[serde(alias = "Ticker")]
    ticker: String,
    #[serde(alias = "Weight")]
    weight: f32,
    #[serde(alias = "Volume")]
    volume: f32,
    #[serde(alias = "UserNameSubmitted")]
    username: String,
    #[serde(alias = "Timestamp")]
    timestamp: String,
}

#[derive(Resource)]
struct ItemsAsset(Handle<Items>);
