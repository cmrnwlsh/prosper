use bevy::{prelude::*, utils::HashMap};

use super::items::Item;

#[derive(Debug)]
pub struct Recipe {
    pub inputs: Vec<(Item, usize)>,
    pub output: usize,
    pub energy: usize,
}

#[derive(Resource, Debug)]
pub struct Recipes(pub HashMap<Item, Recipe>);
impl Default for Recipes {
    fn default() -> Self {
        Self(HashMap::from([]))
    }
}
