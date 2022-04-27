use std::collections::HashMap;
use std::hash::Hash;
use std::clone::Clone;

use winit::{
    event::{ Event, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState }
};

#[derive(Debug)]
pub enum InputObject {
    KeyCode
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum InputEvent {
    Began,
    Changed,
    Ended
}

pub struct Input<I> 
    where I: Fn(InputObject)
{
    handlers: HashMap<InputEvent, I>
}

fn get_inputs<I>  () -> Vec<Input<I>>
    where I: Fn(InputObject) 
{
    Vec::<Input<I>>::new()
}

impl <I: Fn(InputObject)> Input <I> {
    pub fn new() -> Self {
        let input = Self {
            handlers: HashMap::new()
        };

        let mut inputs: Vec<Input<I>> = get_inputs();
        inputs.insert(inputs.len() + 1, input);
        
        input
    }

    pub fn on_input(&mut self, event: InputEvent, callback: I)
        where I: Fn(InputObject)
    {
        self.handlers.insert(event, callback);
    }
}

fn get_all_inputs() {

}

pub fn process_event(event:  &Event<()>) {
    match event {
        Event::WindowEvent { event, .. } => match *event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: VirtualKeyCode,
                    ..
                },
                ..
            } => {
                println!("Pressed {:?}", VirtualKeyCode.unwrap())
            },
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: VirtualKeyCode,
                    ..
                },
                ..
            } => {
                println!("Released {:?}", VirtualKeyCode.unwrap())
            },
            
            _ => {}
        }
        _ => {}
    }
}
