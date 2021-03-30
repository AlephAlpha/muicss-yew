#![recursion_limit = "512"]

mod components;
mod model;
mod pages;
mod switch;

use model::Model;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
