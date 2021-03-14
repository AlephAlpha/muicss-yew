mod components;
mod model;
mod pages;

use model::Model;

fn main() {
    yew::start_app::<Model>();
}
