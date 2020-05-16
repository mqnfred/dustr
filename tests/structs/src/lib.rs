#[macro_use]
extern crate ffishim_derive;
#[ffishim_library]

#[derive(FFIShim)]
pub enum Location {
    GPS(crate::submod::GPS),
    Unknown,
}

#[derive(FFIShim)]
pub struct MyFloat {
    val: f64,
}

mod submod {
    #[derive(FFIShim)]
    pub struct GPS {
        lat: crate::submod::subsubscuba::Latitude,
        lon: f64,
    }

    #[ffishim_use_case]
    fn sub(a: u32, b: u32) -> u32 {
        a - b
    }

    mod subsubscuba {
        #[derive(FFIShim)]
        pub struct Latitude(crate::MyFloat);
    }
}

#[derive(FFIShim)]
pub struct Shy {
    #[ffishim(opaque)]
    introverted: ::std::collections::HashMap<&'static str, u64>,
    extraverted: u64,
}

#[ffishim_use_case]
fn new_shy(extraverted: u64) -> Shy {
    let mut introverted = ::std::collections::HashMap::<&'static str, u64>::default();
    introverted.insert("wow", 1);
    Shy{introverted, extraverted}
}

#[ffishim_use_case]
fn valid_shy(shy: Shy) -> bool {
    shy.introverted.get("wow").is_some()
}

#[derive(FFIShim)]
#[ffishim(opaque)]
pub struct Invisible {
    hidden: u64,
}

#[ffishim_use_case]
fn new_invisible(hidden: u64) -> Invisible {
    Invisible{hidden}
}

#[ffishim_use_case]
fn check_invisible(inv: Invisible) -> bool {
    inv.hidden == 13
}
