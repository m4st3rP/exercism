extern crate chrono;
use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    // I swear to god I tried it like this before and it didn't worked
    start + Duration::seconds(1_000_000_000)
}