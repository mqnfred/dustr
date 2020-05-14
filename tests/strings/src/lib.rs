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
