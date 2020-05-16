#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[ffishim_use_case]
fn add(a: u32, b: u32) -> u32 {
    a + b
}
