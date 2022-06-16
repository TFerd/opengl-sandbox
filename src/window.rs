extern crate gl;
extern crate glfw;

//use gl::types::*;
use glfw::{Action, Context, Key};

//todo
//fn initGlfw() -> glfw::Glfw {
//    create_window();
//}

//TODO: handle inputs
pub fn create_window() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3)); //use OpenGL version 3.3 i think?
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        //???
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    //Creates window and its context?
    let (mut window, events) = glfw
        .create_window(800, 640, "Title", glfw::WindowMode::Windowed)
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
            gl::Viewport(0, 0, 800, 640);
        }
    } else {
        eprintln!("Viewport is NOT loaded!");
    }

    window.set_framebuffer_size_polling(true);

    //Loop until window is closed
    while !window.should_close() {
        //Handle inputs
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    //TODO: configure inputs
                    //Closes window when ESC is pressed i think?
                    window.set_should_close(true);
                }
                _ => {} //otherwise continue/do nothing
            }
        } //End input handler

        //Rendering commands here!?
        if gl::ClearColor::is_loaded() {
            unsafe {
                gl::ClearColor(1.0f32, 0.2f32, 0.5f32, 1.0f32);
            }
        }

        if gl::Clear::is_loaded() {
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
        }

        //poll for and process events ??
        glfw.poll_events();
        //Swap front and back buffers (wtf?)
        window.swap_buffers();
    }
}
