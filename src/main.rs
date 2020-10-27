//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
mod c_expose;
mod gen;
mod generator;
mod result;
mod state;
mod supported_types;
mod utils;

//------------------------------------------------------------------------------
fn main() {
    generator::run(&["example_class.hpp"], "out.hpp", "out.cpp");
}
