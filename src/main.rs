// Prints today's lunar phase

use chrono::{Datelike, Local, TimeZone};
use esbat::daily_lunar_phase;

fn main() {
    let today = Local::today();
    let day = Local.ymd(today.year(), today.month(), today.day());
    let phase = daily_lunar_phase(day);
    print!("{}", phase.as_emoji());
}
