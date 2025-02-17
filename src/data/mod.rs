mod items;
mod recipes;

use bevy::prelude::*;
use recipes::Recipes;

pub struct DataPlugin;
impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Recipes>()
            .add_systems(Startup, |recipes: Res<Recipes>| {
                info!("loaded recipes:\n{:#?}", recipes.0)
            });
    }
}
