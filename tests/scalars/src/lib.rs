#[macro_use]
extern crate ffishim_derive;

#[ffishim_function]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[ffishim_function]
fn not(a: bool) -> bool {
    !a
}

#[ffishim_function]
fn get_some(v: u64) -> Option<u64> {
    Some(v)
}

#[ffishim_function]
fn get_none() -> Option<u64> {
    None
}
