#[macro_use]
extern crate ffishim_derive;

#[ffishim_use_case]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[ffishim_use_case]
fn not(a: bool) -> bool {
    !a
}

#[ffishim_use_case]
fn get_some(v: u64) -> Option<u64> {
    Some(v)
}

#[ffishim_use_case]
fn get_none() -> Option<u64> {
    None
}
