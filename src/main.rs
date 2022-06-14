extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    //Creates window and its context?
    let (mut window, events) = glfw
        .create_window(300, 300, "Title", glfw::WindowMode::Windowed)
        .expect("Failed to create window!");

    //Make the windows context current?
    window.make_current();
    window.set_key_polling(true);

    //Loads all functions pointers
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    //Check if function is loaded before trying to use
    if gl::Viewport::is_loaded() {
        println!("Viewport is loaded");
        unsafe {
            gl::Viewport(0, 0, 300, 300);
        }
    }

    //Loop until window is closed
    while !window.should_close() {
        //Swap front and back buffers (wtf?)
        window.swap_buffers();

        //poll for and process events ??
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    //Closes window when ESC is pressed i think?
                    window.set_should_close(true);
                }
                _ => {} //otherwise continue/do nothing
            }
        }
    }
}
