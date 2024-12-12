use std::fmt::Display;

pub struct Stones(pub Vec<u32>);

impl Display for Stones {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in &self.0 {
            write!(f, "{} ", item)?;
        }
        Ok(())
    }
}

impl From::<&str> for Stones {
    fn from(value: &str) -> Self {
        Self(value
            .trim()
            .split_whitespace()
            .map(|token| { token.parse().unwrap() })
            .collect()
        )
    }
}

impl Stones {
    pub fn blink(self) -> Stones {
        Stones(self.0
            .into_iter()
            .flat_map(|number| {
                vec![ if number == 0 { 1 } else { number }]
            })
            .collect()
        )
    }
}
