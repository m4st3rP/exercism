extern crate chrono;
use chrono::{DateTime, Utc};
use chrono::Datelike;
use chrono::Timelike;
use chrono::TimeZone;

const SECONDS_PER_YEAR: i32 = 31557600;
const SECONDS_PER_MONTH: i32 = 2629800;
const SECONDS_PER_DAY: i32 = 86400;
const SECONDS_PER_HOUR: i32 = 3600;
const SECONDS_PER_MINUTE: i32 = 60;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let mut time_to_process = 1_000_000_000;

	let mut year: i32 = start.year();
	let mut month: i32 = start.month() as i32;
    let mut day: i32 = start.day() as i32;
    let mut hours: i32 = start.hour() as i32;
    let mut minutes: i32 = start.minute() as i32;
    let mut seconds: i32 = start.second() as i32;

    let years_to_add: i32 = time_to_process / SECONDS_PER_YEAR;
    year += years_to_add;
    time_to_process -= years_to_add * SECONDS_PER_YEAR;

    let months_to_add = time_to_process / SECONDS_PER_MONTH;
    month += months_to_add;
    time_to_process -= months_to_add * SECONDS_PER_MONTH;

    let days_to_add = time_to_process / SECONDS_PER_DAY;
    day += days_to_add;
    time_to_process -= days_to_add * SECONDS_PER_DAY;

    let hours_to_add = time_to_process / SECONDS_PER_HOUR;
    hours += hours_to_add;
    time_to_process -= hours_to_add * SECONDS_PER_HOUR;

    let days_to_add = time_to_process / SECONDS_PER_MINUTE;
    minutes += days_to_add;
    time_to_process -= days_to_add * SECONDS_PER_MINUTE;

    seconds += time_to_process;

    Utc.ymd(year, month as u32, day as u32).and_hms(hours as u32, minutes as u32, seconds as u32)
}