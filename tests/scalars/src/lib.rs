#[macro_use]
extern crate ffishim_derive;

#[ffishim_use_case]
fn add_i64(a: i64, b: i64) -> i64 {
    a + b
}

#[ffishim_use_case]
fn add_f64(a: f64, b: f32) -> f64 {
    a + b as f64
}

#[ffishim_use_case]
fn fails(a: i64) -> Result<i64, ::ffishim::library::Error> {
    if a == 5 {
        Ok(a + 1)
    } else {
        Err(::ffishim::library::Error::msg("only 5 accepted"))
    }
}
