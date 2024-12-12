use std::fmt::Display;

pub struct PuzzleInput(Vec<u32>);

impl Display for PuzzleInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            write!(f, "{} ", item)?;
        }
        Ok(())
    }
}

impl From::<&str> for PuzzleInput {
    fn from(value: &str) -> Self {
        Self(value
            .trim()
            .split_whitespace()
            .map(|token| { token.parse().unwrap() })
            .collect()
        )
    }
}
