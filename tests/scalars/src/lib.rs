#[macro_use]
extern crate ffishim_derive;

#[ffishim_library]
#[ffishim_use_case]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[derive(FFIShim)]
pub struct GPS {
    lat: f64,
    lon: f64,
}

/*
#[ffishim_use_case]
fn add_to_latitude(mut gps: GPS, lat_diff: f64) -> GPS {
    gps.lat += lat_diff;
    gps
}
*/
