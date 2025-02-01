use bevy::prelude::*;
use serde::Deserialize;

use crate::plugin::sim::components::Material;

#[derive(Deserialize, Debug, Bundle)]
pub struct Wearable {
    material: Material,
}
