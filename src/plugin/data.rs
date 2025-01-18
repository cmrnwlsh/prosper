use bevy::prelude::*;
use static_toml::static_toml;

pub fn plugin(app: &mut App) {
    static_toml! {
        #[derive(Debug)]
        static ITEMS = include_toml!("data/items.toml");
    }
    app.add_systems(Startup, || {
        info!("{:#?}", ITEMS);
    });
}
