use chrono::{DateTime, Utc};
use itertools::Itertools;
use std::{fs, path::Path};

use crate::{currency::Amount, Result};

#[derive(Debug, serde::Deserialize)]
enum DebitOrCredit {
    DBIT,
    CRDT,
}

#[derive(Debug, serde::Deserialize)]
enum Status {
    PENDING,
    BILLED,
}

#[derive(Debug, serde::Deserialize)]
struct Row {
    #[serde(rename = "Transaction Date", with = "virgin_date_format")]
    transaction_date: DateTime<Utc>,

    #[serde(rename = "Posting Date", with = "virgin_date_format")]
    posting_date: DateTime<Utc>,

    #[serde(rename = "Billing Amount")]
    billing_amount: Amount,

    #[serde(rename = "Merchant")]
    merchant: String,

    #[serde(rename = "Merchant City")]
    merchant_city: String,

    #[serde(rename = "Merchant State")]
    merchant_state: String,

    #[serde(rename = "Merchant Postcode")]
    merchant_postcode: String,

    #[serde(rename = "Reference Number")]
    reference_number: String,

    #[serde(rename = "Debit or Credit")]
    debit_or_credit: DebitOrCredit,

    #[serde(rename = "SICMCC Code")]
    sicmcc_code: String,

    #[serde(rename = "Status")]
    status: Status,

    #[serde(rename = "Transaction Currency")]
    transaction_currency: String,

    #[serde(rename = "Additional Card Holder")]
    additional_card_holder: bool,

    #[serde(rename = "Card Used")]
    card_used: String,
}

const MERCHANT_FIELD_NO: usize = 3;
const NO_OF_FIELDS: usize = 14;

pub async fn run(batch_file: String) -> Result<()> {
    let batch_path = Path::new(&batch_file);
    let csv_content = fs::read_to_string(batch_path)?;

    let lines: String = csv_content.lines().map(|line| clean_line(line)).join("\n");

    let mut reader = csv::Reader::from_reader(lines.as_bytes());
    for result in reader.deserialize() {
        let _row: Row = result?;
    }

    Ok(())
}

fn clean_line(line: &str) -> String {
    let mut fields: Vec<&str> = line.split(",").collect();
    if fields.len() > NO_OF_FIELDS {
        println!("bad row found, correcting. line='{}'", line);
    }

    let no_of_additional_fields = fields.len() - NO_OF_FIELDS;
    let mut first_fields: Vec<&str> = fields.drain(0..MERCHANT_FIELD_NO).collect();
    let merchant_field: String = fields.drain(0..no_of_additional_fields + 1).join(" ");

    first_fields.push(&merchant_field);

    first_fields.into_iter().chain(fields.into_iter()).join(",")
}

mod virgin_date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}
