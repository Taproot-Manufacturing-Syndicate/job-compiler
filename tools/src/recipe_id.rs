use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct RecipeId {
    pub repo: Option<String>,
    pub recipe: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum RecipeIdParseError {
    #[error("syntax error: {0}")]
    SyntaxError(String),
}

impl FromStr for RecipeId {
    type Err = RecipeIdParseError;
    fn from_str(recipe_id: &str) -> Result<RecipeId, Self::Err> {
        let substrings: Vec<&str> = recipe_id.split(':').collect();
        match substrings.len() {
            1 => Ok(RecipeId {
                repo: None,
                recipe: String::from(recipe_id),
            }),
            2 => Ok(RecipeId {
                repo: Some(String::from(substrings[0])),
                recipe: String::from(substrings[1]),
            }),
            _ => Err(RecipeIdParseError::SyntaxError(String::from(recipe_id))),
        }
    }
}

mod test {
    #[cfg(test)]
    use crate::recipe_id::{FromStr, RecipeId, RecipeIdParseError};

    #[test]
    fn parse_simple() {
        let r = RecipeId::from_str("hello");
        assert_eq!(
            r,
            Ok(RecipeId {
                repo: None,
                recipe: "hello".into(),
            })
        );
    }

    #[test]
    fn parse_with_repo() {
        let r = RecipeId::from_str("my_repo:my_recipe");
        assert_eq!(
            r,
            Ok(RecipeId {
                repo: Some("my_repo".into()),
                recipe: "my_recipe".into(),
            })
        );
    }

    #[test]
    fn parse_too_many_colons() {
        let r = RecipeId::from_str("my_repo:my_recipe:something_else");
        assert_eq!(
            r,
            Err(RecipeIdParseError::SyntaxError(
                "my_repo:my_recipe:something_else".into()
            ))
        );
    }
}
