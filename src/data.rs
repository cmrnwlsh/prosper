use bevy::{prelude::*, utils::HashMap};
use bevy_common_assets::msgpack::MsgPackAssetPlugin;
use serde::Deserialize;

static DATA_PATH: &str = "embedded://data.mpk";

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MsgPackAssetPlugin::<Data>::new(&[DATA_PATH]))
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
    items: HashMap<String, Item>,
    buildings: HashMap<String, Building>,
    recipes: HashMap<String, Recipe>,
}

#[derive(Resource)]
struct DataHandle(Handle<Data>);

#[derive(Deserialize, Debug)]
pub enum Expertise {
    Agriculture,
    Chemistry,
    Construction,
    Electronics,
    FoodIndustries,
    FuelRefining,
    Manufacturing,
    Metallurgy,
    ResourceExtraction,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    category_name: String,
    name: String,
    volume: f32,
    weight: f32,
}

#[derive(Deserialize, Debug)]
pub struct Building {
    area_cost: i32,
    building_costs: Vec<Commodity>,
    expertise: Option<Expertise>,
    name: String,
    recipes: Vec<String>,
    pioneers: i32,
    engineers: i32,
    technicians: i32,
    settlers: i32,
    scientists: i32,
}

#[derive(Deserialize, Debug)]
pub struct Recipe {
    building_ticker: String,
    inputs: Vec<Commodity>,
    outputs: Vec<Commodity>,
    time_ms: u64,
}

#[derive(Deserialize, Debug)]
pub struct Commodity {
    amount: i32,
    ticker: String,
}
