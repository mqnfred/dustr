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
