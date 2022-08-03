extern crate gl;
extern crate glfw;
//extern crate opengl;

use core::mem::size_of;
use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Key};
use opengl::buffer::*;
use opengl::shader::*;
use opengl::vertex_array::*;
use opengl::*;

type Vertex = [f32; 3];
const VERTICES: [Vertex; 4] = [
    // A rectangle has 4 points
    [0.5, 0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
];

type TriIndexes = [u32; 3];
const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 final_color;
  void main() {
    final_color = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

//todo
//fn initGlfw() -> glfw::Glfw {
//    create_window();
//}

// TODO: handle inputs
pub fn create_window() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3)); // use OpenGL version 3.3 i think?
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        //???
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Creates window and its context?
    let (mut window, events) = glfw
        .create_window(800, 640, "Title", glfw::WindowMode::Windowed)
        .expect("Failed to create window!");

    // Make the windows context current?
    window.make_current();
    window.set_key_polling(true);

    window.set_framebuffer_size_polling(true);

    // Set VSync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    // Loads all functions pointers
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    // Check if function is loaded before trying to use
    if gl::Viewport::is_loaded() {
        println!("Viewport is loaded");
        unsafe {
            gl::Viewport(0, 0, 800, 640);
        }
    } else {
        eprintln!("Viewport is NOT loaded!");
    }

    //????????
    window.set_framebuffer_size_polling(true);

    opengl::clear_color(1.0, 0.2, 0.5, 1.0);

    // Generate a Vertex Array Object(VAO)
    let vao = VertexArray::new().expect("Failed to create VAO");
    vao.bind();

    // Generate Vertex Buffer Objects (VBOs)
    let vbo = Buffer::new().expect("Failed to create VBO");
    vbo.bind(BufferType::Array);
    Buffer::buffer_data(
        BufferType::Array,
        bytemuck::cast_slice(&VERTICES),
        gl::STATIC_DRAW,
    );

    // Create Element Buffer Object (EBO)
    let ebo = Buffer::new().expect("Failed to create EBO");
    ebo.bind(BufferType::ElementArray);
    Buffer::buffer_data(
        BufferType::ElementArray,
        bytemuck::cast_slice(&INDICES),
        gl::STATIC_DRAW,
    );

    unsafe {
        // Vertex Attribute
        gl::VertexAttribPointer(
            0, // bc we put: layout (location = 0) in our vertex code
            3, // bc our vertex is a vec3 (x, y, z)
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        gl::EnableVertexAttribArray(0);
    }

    let program = ShaderProgram::create_program_from_src(VERT_SHADER, FRAG_SHADER).unwrap(); //unwrap bc its a Result
    program.use_program();

    let mut wireframe_mode = false;
    /*********************************************************************************************************
    @TODO: https://learnopengl.com/Getting-started/Hello-Triangle
    This says that UseProgram, BindVertexArray, DrayElements, and BindVertexArray should be in the render loop
    and everything else should be in the initialization (before the loop)
    **********************************************************************************************************/
    // Rendering Loop:
    while !window.should_close() {
        // Handle inputs
        process_events(&mut window, &events, &mut wireframe_mode);

        // Rendering commands here!?
        // Yes, once the events are clear, we can change the world state.
        // Then, draw

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }

        // poll for and process events ??
        glfw.poll_events();
        // Swap front and back buffers (wtf?)
        window.swap_buffers();
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    wireframe_mode: &mut bool, // Get this outta here, maybe pass in an options object
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },

            // Esc | Close window
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                // Closes window when ESC is pressed i think? yes
                window.set_should_close(true);
            }
            // Spacebar | Toggle wireframe mode
            glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                if *wireframe_mode == false {
                    opengl::polygon_mode(PolygonMode::Line);
                    *wireframe_mode = true;
                } else {
                    opengl::polygon_mode(PolygonMode::Fill);
                    *wireframe_mode = false;
                }
            }
            _ => {} // otherwise continue/do nothing
        }
    }
}

/*
fn create_vertex(vertices: [i32; 9]) {
    let mut VBO: u32;

    if !gl::GenBuffers::is_loaded(){

    }
}
*/
