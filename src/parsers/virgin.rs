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
    #[serde(rename = "Transaction Date")]
    transaction_date: String,

    #[serde(rename = "Posting Date")]
    posting_date: String,

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

#[allow(unstable_name_collisions)]
pub async fn run(batch_file: String) -> Result<()> {
    let batch_path = Path::new(&batch_file);
    let csv_content = fs::read_to_string(batch_path)?;

    let lines: String = csv_content
        .lines()
        .map(|line| clean_line(line))
        .intersperse(String::from("\n"))
        .collect();

    let mut reader = csv::Reader::from_reader(lines.as_bytes());
    for result in reader.deserialize() {
        let row: Row = result?;
    }

    Ok(())
}

#[allow(unstable_name_collisions)]
fn clean_line(line: &str) -> String {
    let mut fields: Vec<&str> = line.split(",").collect();
    if fields.len() > NO_OF_FIELDS {
        println!("bad row found, correcting. line='{}'", line);
    }

    let no_of_additional_fields = fields.len() - NO_OF_FIELDS;
    let mut first_fields: Vec<&str> = fields.drain(0..MERCHANT_FIELD_NO).collect();
    let merchant_field: String = fields
        .drain(0..no_of_additional_fields + 1)
        .intersperse(" ")
        .collect();

    first_fields.push(&merchant_field);

    first_fields
        .into_iter()
        .chain(fields.into_iter())
        .intersperse(",")
        .collect()
}
