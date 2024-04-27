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

// [destinations.personal]
// account = "998877665"
// source = "personal"

// [destinations.joint]
// account = "112233445"
// source = "joint"
