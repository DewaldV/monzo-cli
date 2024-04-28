use std::{
    fmt::Display,
    num::{ParseIntError, TryFromIntError},
    str::FromStr,
};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "&str")]
pub struct Amount {
    pub pence: i64,
}

impl From<i64> for Amount {
    fn from(pence: i64) -> Self {
        Amount { pence }
    }
}

impl FromStr for Amount {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned_s = s.replace(&['.', ','], "");
        let pence = cleaned_s.parse()?;
        Ok(Amount { pence })
    }
}

impl TryFrom<&str> for Amount {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Amount::from_str(value)
    }
}

impl TryInto<u32> for Amount {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<u32, Self::Error> {
        self.pence.try_into()
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();

        for (i, c) in self.pence.to_string().chars().rev().enumerate() {
            if i == 2 {
                out.push('.');
            }

            if i > 2 && (i - 2) % 3 == 0 {
                out.push(',');
            }

            out.push(c);
        }

        let out: String = out.chars().rev().collect();

        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn format_currency_decimals() {
        let pence = 890;

        let value = Amount { pence }.to_string();

        assert_eq!(value, "8.90")
    }

    #[test]
    fn format_currency_separators() {
        let cases = [
            (12345, "123.45"),
            (654321, "6,543.21"),
            (500000035, "5,000,000.35"),
        ];

        for (pence, expected) in cases {
            let value = Amount { pence }.to_string();

            assert_eq!(value, expected);
        }
    }

    #[test]
    fn parse_currency_decimals() {
        let pence = "10.05";

        let value = Amount::try_from(pence)
            .expect("must parse fixed value")
            .pence;

        assert_eq!(value, 1005);
    }

    #[test]
    fn parse_currency_separators() {
        let pence = "1,010.05";

        let value = Amount::try_from(pence)
            .expect("must parse fixed value")
            .pence;

        assert_eq!(value, 101005);
    }

    #[test]
    fn from_i64() {
        let pence: i64 = 953578513525;

        let value = Amount::from(pence);

        assert_eq!(value.pence, pence);
    }

    #[test]
    fn try_into_u32() {
        let pence = 1024;

        let res: Result<u32, _> = Amount { pence }.try_into();

        assert_eq!(res.is_ok(), true);
    }
}
