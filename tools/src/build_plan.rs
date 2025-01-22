// This is a representation of the recipes/steps that result in a
// desired Output.  It's a path from starting points/vitamins to the
// desired Output.

use crate::quantity::*;

// FIXME: this isn't the correct abstraction for Item
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub quantity: Quantity,
}

#[derive(Debug)]
pub struct BuildPlan<'a> {
    // Keys are the names of Items.
    // Values are Items, which are just the name and a Quantity.
    pub bom: std::collections::HashMap<String, Item>,

    pub repos: &'a crate::repos::Repos,
}

impl<'a> BuildPlan<'a> {
    pub fn new(repos: &'a crate::repos::Repos) -> Self {
        BuildPlan {
            bom: std::collections::HashMap::<String, Item>::new(),
            repos,
        }
    }
}
