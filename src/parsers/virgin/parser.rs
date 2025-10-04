use chrono::NaiveDate;
use itertools::Itertools;
use std::{fs, path::Path};

use crate::Result;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum DebitOrCredit {
    DBIT,
    CRDT,
}

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize)]
enum SICMCCCode {
    PR,
    PY,
    FE,
}

#[derive(PartialEq, Debug, serde::Deserialize, serde::Serialize)]
enum Status {
    PENDING,
    BILLED,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Row {
    #[serde(rename = "Transaction Date", with = "virgin_date_format")]
    transaction_date: NaiveDate,

    #[serde(
        rename = "Posting Date",
        with = "virgin_date_format_optional",
        skip_serializing_if = "Option::is_none"
    )]
    posting_date: Option<NaiveDate>,

    #[serde(rename = "Billing Amount")]
    billing_amount: f64,

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

    #[serde(rename = "SICMCC Code", skip_serializing_if = "Option::is_none")]
    sicmcc_code: Option<SICMCCCode>,

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

pub async fn run(batch_file: String, filter_date: NaiveDate, output_file: String) -> Result<()> {
    println!("starting virgin credit card parse, file={}", &batch_file);
    let batch_path = Path::new(&batch_file);
    let csv_content = fs::read_to_string(batch_path)?;

    let lines: String = csv_content.lines().map(clean_line).join("\n");

    let mut reader = csv::Reader::from_reader(lines.as_bytes());

    let rows: Vec<Row> = reader.deserialize().collect::<csv::Result<Vec<Row>>>()?;

    let filtered_rows: Vec<Row> = rows
        .into_iter()
        .filter(|r| {
            r.posting_date.is_some_and(|date| date >= filter_date)
                && r.status == Status::BILLED
                && r.sicmcc_code == Some(SICMCCCode::PR)
        })
        .collect();

    println!("successfully filtered rows. count={}", filtered_rows.len());

    // Write the filtered rows to a new CSV file
    let mut wtr = csv::Writer::from_path(output_file)?;

    for row in filtered_rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;

    Ok(())
}

fn clean_line(line: &str) -> String {
    let mut fields: Vec<&str> = line.split(",").collect();

    if fields.len() <= NO_OF_FIELDS {
        return line.to_string();
    }

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
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod virgin_date_format_optional {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match date {
            Some(d) => format!("{}", d.format(FORMAT)),
            None => String::from("null"),
        };
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "null" {
            return Ok(None);
        }

        NaiveDate::parse_from_str(&s, FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
