use rand::{seq::SliceRandom, thread_rng, Rng};
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Username(String);

impl TryFrom<String> for Username {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_empty_or_whitespace = value.trim().is_empty();
        let is_too_long = value.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters =
            value.chars().any(|c| forbidden_characters.contains(&c));
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            return Err(format!("{} is not a valid subscriber name.", value));
        }
        Ok(Self(value))
    }
}

impl Default for Username {
    fn default() -> Self {
        let adjectives = [
            "swift", "bright", "clever", "brave", "mighty", "noble", "wise", "calm", "kind",
            "bold", "quick", "sharp", "smart", "keen", "fair",
        ];

        let nouns = [
            "wolf", "eagle", "lion", "hawk", "bear", "tiger", "fox", "owl", "deer", "seal",
            "raven", "crane", "dove", "swan", "falcon",
        ];

        let mut rng = thread_rng();

        let adjective = adjectives.choose(&mut rng).unwrap();
        let noun = nouns.choose(&mut rng).unwrap();

        let number = rng.gen_range(100..1000);

        let username = format!("{}_{}_{}", adjective, noun, number);

        Self(username)
    }
}

impl FromStr for Username {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s.to_owned())
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
