use std::num::ParseIntError;

pub fn format_currency(pence: i64) -> String {
    let mut out: String = String::new();

    for (i, c) in pence.to_string().chars().rev().enumerate() {
        if i == 2 {
            out.push('.');
        }

        if i > 2 && (i - 2) % 3 == 0 {
            out.push(',');
        }

        out.push(c);
    }

    out.chars().rev().collect()
}

pub fn parse_currency(value: &str) -> Result<u32, ParseIntError> {
    let cleaned_value = value.replace(&['.', ','], "");
    cleaned_value.parse()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_currency_decimals() {
        let pence = 890;

        let value = format_currency(pence);

        assert_eq!(value, "8.90")
    }

    #[test]
    fn format_currency_separators() {
        let cases = [
            (12345, "123.45"),
            (654321, "6,543.21"),
            (500000035, "5,000,000.35"),
            (953578513525, "9,535,785,135.25"),
        ];

        for (pence, expected) in cases {
            let value = format_currency(pence);

            assert_eq!(value, expected);
        }
    }

    #[test]
    fn parse_currency_decimals() {
        let value = "10.05";

        let pence = parse_currency(value);

        assert_eq!(pence, Ok(1005));
    }

    #[test]
    fn parse_currency_separators() {
        let value = "1,010.05";

        let pence = parse_currency(value);

        assert_eq!(pence, Ok(101005));
    }
}
