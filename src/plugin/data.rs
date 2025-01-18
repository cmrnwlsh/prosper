use bevy::prelude::*;
use static_toml::static_toml;

static_toml! {
    #[derive(Debug)]
    static ITEMS = include_toml!("data/items.toml");
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, || {
        info!("{:#?}", ITEMS);
    });
}
