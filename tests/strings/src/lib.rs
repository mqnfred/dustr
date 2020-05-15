#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]
#[derive(FFIShim)]
pub struct Address {
    line: String,
    city: String,
    country: String,
}

#[ffishim_use_case]
fn set_address_line(mut addr: Address, line: String) -> Address {
    addr.line = line;
    addr
}

// TODO: the dart side leaks this string in the wrapper
// function. this is a real bug that needs fixing. would
// likely imply wrapping the string on the dart side
#[ffishim_use_case]
fn get_address_line(addr: Address) -> String {
    addr.line
}
