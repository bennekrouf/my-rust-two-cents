#![allow(unused)]
use time::{Duration, macros::date, macros::time};
use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let b:i64 = 1000000000;
    let d = Duration::seconds(b + start.time().second() as i64);
    start + d
}

pub fn main() {
    println!("{:?}", after(DateTime::new(date!(2015-01-24), time!(23:59))));
}