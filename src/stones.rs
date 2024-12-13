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
        fn count_digits(number: u32) -> u32 {
            if number != 0 {
                1 + count_digits(number / 10)
            }
            else {
                0
            }
        }

        Stones(self.0
            .into_iter()
            .flat_map(|number| {
                if number == 0 { vec![1] }
                else {
                    let digits = count_digits(number);
                    if digits % 2 == 0 { 
                        let left = number / u32::pow(10, digits / 2);
                        let right = number % u32::pow(10, digits / 2);

                        vec![left, right] 
                    } else { vec![number * 2024] }
                }
            })
            .collect()
        )
    }
}
