extern crate gl;
extern crate glfw;
extern crate image;
use image::GenericImage;

use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use std::sync::mpsc::Receiver;

use gl::types::*;
use glfw::{Action, Context, Key};
use opengl::buffer::*;
use opengl::shader::*;
use opengl::texture::*;
use opengl::vertex_array::*;
use opengl::*;

type Vertex = [f32; 8]; //First three are vertex location, 4 5 and 6 are the color values, 7 and 8 are texture coords
const VERTICES: [Vertex; 4] = [
    [0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
    [0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    [-0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0],
];

const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

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
        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;
        // position attribute
        gl::VertexAttribPointer(
            0, // bc we put: layout (location = 0) in our vertex code
            3, // bc our vertex is a vec3 (x, y, z)
            gl::FLOAT,
            gl::FALSE,
            stride,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // color attribute
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(1);

        // texture attribute
        gl::VertexAttribPointer(
            2,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (6 * mem::size_of::<GLfloat>()) as *const c_void,
        );
        gl::EnableVertexAttribArray(2);
    }
    // Configuring Textures
    let texture = Texture::new().expect("Failed to create texture!");
    texture.bind(TextureType::Texture2D);

    // texture wrapping
    set_parameter(
        TextureType::Texture2D,
        TextureOption::TextureWrapS,
        TextureOptionValue::Repeat,
    );
    set_parameter(
        TextureType::Texture2D,
        TextureOption::TextureWrapT,
        TextureOptionValue::Repeat,
    );

    // texture filtering
    set_parameter(
        TextureType::Texture2D,
        TextureOption::MinFilter,
        TextureOptionValue::Linear,
    );
    set_parameter(
        TextureType::Texture2D,
        TextureOption::MagFilter,
        TextureOptionValue::Linear,
    );

    // Load texture image
    let img = image::open(&Path::new("resources/container.jpg")).expect("Failed to load texture.");
    let img_data = img.raw_pixels();

    tex_image_2d(
        TextureType::Texture2D,
        0,
        InternalFormat::RGB,
        img.width(),
        img.height(),
        InternalFormat::RGB,
        &img_data[0],
    );

    generate_mipmap(TextureType::Texture2D);

    let program =
        ShaderProgram::create_program_from_files("src/VertexShader.vs", "src/FragmentShader.fs")
            .unwrap();

    let mut wireframe_mode = false;

    /*********************************************************************************************************
    @TODO: https://learnopengl.com/Getting-started/Hello-Triangle
    This says that UseProgram, BindVertexArray, DrayElements, and BindVertexArray should be in the render loop
    and everything else should be in the initialization (before the loop)
    **********************************************************************************************************/
    // Rendering Loop:
    while !window.should_close() {
        // Handle inputs
        process_events(&mut window, &events, &mut wireframe_mode); //Maybe in the future, add options object as third param

        // Rendering commands here!?
        // Yes, once the events are clear, we can change the world state.
        // Then, draw

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            texture.bind(TextureType::Texture2D);
            program.use_program();
            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);
        }

        // poll for and process events ??
        glfw.poll_events();
        // Swap front and back buffers (wtf?) one buffer draws the other calculates?
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
