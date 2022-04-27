use crate::input_controller;
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ScanCode, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{
        WindowBuilder,
    },
    dpi::{
        LogicalSize,
    }
};

pub fn init(name: &str, x: u32, y: u32) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(name);
    window.set_inner_size(LogicalSize::new(x, y));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        input_controller::process_event(&event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },

            Event::MainEventsCleared => {
                // Application update code.
    
                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }
    });
}