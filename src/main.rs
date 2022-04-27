mod window;
mod input_controller;

use input_controller::{ InputObject, InputEvent, Input};

fn main() {
    let _ = window::init("MyWindow", 600, 400);
    let mut input = Input::new();
    input.on_input(InputEvent::Began, | input_object: InputObject | {
        println!("{:?}", input_object)
    });
}