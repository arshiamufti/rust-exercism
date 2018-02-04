extern crate chrono;
use chrono::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond = 1000000000;
    start.checked_add_signed(Duration::seconds(gigasecond)).unwrap()
}
