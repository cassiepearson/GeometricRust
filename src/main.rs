#![feature(proc_macro_hygiene)] // Allow inline-python code to work using proc macros
#![allow(dead_code)]

mod errors;
mod traits;
mod types;

fn main() {
    println!("It is a main");
}
