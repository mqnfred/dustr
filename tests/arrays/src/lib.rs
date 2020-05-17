#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[ffishim_use_case]
fn new_u64_array(first: u64, second: u64) -> Vec<u64> {
    vec![first, second]
}
