extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        let window = glfw::Window::create(400, 400, "English 日本語 русский язык 官話", glfw::Windowed).unwrap();
        
        window.make_context_current();
        glfw::set_swap_interval(1);
        
        while window.get_param(glfw::SHOULD_CLOSE) == glfw::FALSE {
            glfw::wait_events();
        }
        
        window.destroy();
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}