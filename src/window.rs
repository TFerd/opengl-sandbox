extern crate gl;
extern crate glfw;

use core::mem::{size_of, size_of_val};

//use gl::types::*;
use glfw::{Action, Context, Key};

type Vertex = [f32; 3];
const VERTICES: [Vertex; 4] = [
    [0.5, -0.5, 0.0],
    [0.5, -0.5, 0.0],
    [-0.5, -0.5, 0.0],
    [-0.5, 0.5, 0.0],
];
const INDICES: [u32; 6] = [0, 1, 3, 1, 2, 3];

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

    //Set VSync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

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

    //????????
    window.set_framebuffer_size_polling(true);

    unsafe {
        gl::ClearColor(1.0, 0.2, 0.5, 1.0);

        //Generate a Vertex Array Object(VAO)
        /*
        let mut vao: u32 = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
        */
        let vao = opengl::VertexArray::new().expect("Couldn't make a VAO");
        vao.bind();

        //Generate Vertex Buffer Objects (VBOs)
        let mut vbo: u32 = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo); //Set this buffer (vbo) as active, meaning
        gl::BufferData(
            //this is where we will be putting data next (i think)
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        //Create Element Buffer Object (EBO)
        let mut ebo = 0;
        gl::GenBuffers(1, &mut ebo);
        assert_ne!(ebo, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(&INDICES) as isize,
            INDICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );

        //Vertex Attribute
        gl::VertexAttribPointer(
            0, //bc we put: layout (location = 0) in our vertex code
            3, //bc our vertex is a vec3 (x, y, z)
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );

        gl::EnableVertexAttribArray(0);

        //Create Vertex Shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(vertex_shader);
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        //Check vertex shader compile success
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));

            /*
            let buffer_size = gl::INFO_LOG_LENGTH;
            let mut v: Vec<u8> = Vec::with_capacity(buffer_size.try_into().unwrap());
            let mut log_len = 0_i32;

            gl::GetShaderInfoLog(
                vertex_shader,
                buffer_size.try_into().unwrap(),
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v)); */
        } //End Vertex Shader

        //Create Fragment Shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        gl::CompileShader(fragment_shader);

        //Check fragment shader compile success
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        } //End fragment shader

        //Create program
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        //Check for link error
        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        } //End create program

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::UseProgram(shader_program);
    }

    let mut wireframe_mode = false;
    //Loop until window is closed
    while !window.should_close() {
        //Handle inputs
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                //Esc | Close window
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    //Closes window when ESC is pressed i think? yes
                    window.set_should_close(true);
                }
                //Spacebar | Toggle wireframe mode
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    println!("Wireframe mode is currently: {}", wireframe_mode);
                    if !wireframe_mode {
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                        }
                        wireframe_mode = true;
                    } else {
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                        }
                        wireframe_mode = false;
                    }
                }
                _ => {} //otherwise continue/do nothing
            }
        } //End input handler

        //Rendering commands here!?

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            //useprogram
            //bindvertexarray(VAO[0])
            //drawArrays(gltriangles,0,3);
            //bindvertexarray(vao[1])
            //drawarrays(gltriangles,0,3);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const _);
        }

        //poll for and process events ??
        glfw.poll_events();
        //Swap front and back buffers (wtf?)
        window.swap_buffers();
    }
}

/*
fn create_vertex(vertices: [i32; 9]) {
    let mut VBO: u32;

    if !gl::GenBuffers::is_loaded(){

    }
}
*/
