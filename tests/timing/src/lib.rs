#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[derive(FFIShim)]
pub struct WorldRecord {
    time: ::chrono::Duration,
}

#[ffishim_use_case]
fn get_days_duration_in_ms(days: i64) -> ::chrono::Duration {
    ::chrono::Duration::days(days)
}