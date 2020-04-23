#[derive(Debug)]
pub struct Module {
    name: String,
    datas: Vec<::ffishim::Data>,
    functions: Vec<::ffishim::Function>,
    subs: Vec<Module>,
}
mod module;

#[derive(Debug)]
pub struct Library {
    name: String,
    subs: Vec<Library>,
}
mod library;

#[derive(Debug)]
pub struct Package {
    name: String,
    libraries: Vec<Library>,
}
//mod package;

pub mod types;

#[macro_use]
mod helpers;
