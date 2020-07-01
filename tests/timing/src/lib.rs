#[macro_use]
extern crate ffishim_derive;

#[derive(FFIShim)]
pub struct WorldRecord {
    time: ::chrono::Duration,
}

#[ffishim_function]
fn get_days_duration_in_ms(days: i64) -> ::chrono::Duration {
    ::chrono::Duration::days(days)
}
