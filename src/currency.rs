#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_currency_decimals() {
        let pence: i64 = 890;

        let value: String = format_currency(pence);

        assert_eq!(value, "8.90")
    }

    #[test]
    fn format_currency_separators() {
        let cases = [
            (654321, "6,543.21"),
            (500000035, "5,000,000.35"),
            (953578513525, "9,535,785,135.25"),
        ];

        for (pence, expected) in cases {
            let value = format_currency(pence);

            assert_eq!(value, expected);
        }
    }
}

pub fn format_currency(pence: i64) -> String {
    let mut out: String = String::new();

    for (i, c) in pence.to_string().chars().rev().enumerate() {
        if i == 2 {
            out.push('.')
        }

        if i > 2 && (i - 2) % 3 == 0 {
            out.push(',')
        }

        out.push(c)
    }

    out.chars().rev().collect()
}
