use serde::Deserialize;

use crate::accounts::AccountType;

#[derive(Deserialize)]
struct Destinations {
    destinations: Vec<Destination>,
}

#[derive(Deserialize)]
struct Destination {
    source: AccountType,
    pot_name: String,
}

// [[destinations]]
// source = "personal"
// pot_name = "personal-test-pot"

// [[destinations]]
// source = "joint"
// pot_name = "joint-test-pot"
