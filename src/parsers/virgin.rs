// Transaction Date,Posting Date,Billing Amount,Merchant,Merchant City,Merchant State,Merchant Postcode,Reference Number,Debit or Credit,SICMCC Code,Status,Transaction Currency,Additional Card Holder,Card Used

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

const NO_OF_FIELDS: usize = 14;
const NO_OF_FIELDS_AFTER_MERCHANT: usize = 10;
const MERCHANT_FIELD_IDX: usize = 3;

pub async fn run(batch_file: String) -> Result<()> {
    let batch_path = Path::new(&batch_file);
    let csv_content = fs::read_to_string(batch_path)?;

    for line in csv_content.lines() {
        let mut fields: Vec<&str> = line.split(",").collect();
        if fields.len() > NO_OF_FIELDS {
            println!("bad row found, correcting. line='{}'", line);

            let no_of_additional_fields = fields.len() - NO_OF_FIELDS;
            println!("no_of_additional_fields={}", no_of_additional_fields);

            let first_fields: Vec<&str> = fields.drain(0..2).collect();
            let merchant_field = fields
                .drain(0..no_of_additional_fields)
                .collect::<Vec<&str>>()
                .join(" ");

            let new_line = first_fields
                .into_iter()
                .chain(merchant_field.into_iter())
                .chain(fields.into_iter());

            // 0,1,2
            // Rebuild field 3
            // Slice 3..<no_of_bad_fields>
            // Find the last bad field = len() - 10
        }
    }

    // let csv_lines Vec<Split<String>> = csv_content.lines().map(|l| l.split(",")).collect();
    // let bad_lines = csv_lines.filter(|l| l.len() > 14);

    // let mut reader = csv::Reader::from_path(batch_path)?;
    // for result in reader.deserialize() {
    //     let row: Row = result?;
    //     dbg!(row);
    // }

    Ok(())
}
