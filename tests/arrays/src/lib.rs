#[macro_use]
extern crate ffishim_derive;

#[ffishim_function]
fn new_u64_array(first: u64, second: u64) -> Vec<u64> {
    vec![first, second]
}

#[derive(FFIShim)]
pub struct Crowd {
    members: Vec<Person>,
}

#[derive(FFIShim)]
pub struct Person {
    points: Vec<u64>,
    tagline: String,
}

#[ffishim_function]
fn create_crowd() -> Crowd {
    Crowd{
        members: vec![
            Person{
                points: vec![1, 3, 5],
                tagline: "I like hapricots".to_owned(),
            },
            Person{
                points: vec![2, 4, 6],
                tagline: "Yes, we can".to_owned(),
            },
        ],
    }
}
