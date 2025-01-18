use crate::recipe::Recipe;

pub struct Repo {
    path: String,
    recipes: std::collections::HashMap<String, Recipe>,
}
