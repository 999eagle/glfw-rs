extern mod glfw3;

fn main() {
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn {
        
        glfw3::set_error_callback(error_callback);
        
        if !glfw3::init() {
            glfw3::terminate();
            die!(~"Failed to initialize GLFW\n");
        }
        
        let window =
            match glfw3::Window::create(300, 300, "Hello this is window", glfw3::Windowed) {
                Some(w) => { w }
                None => {
                    glfw3::terminate();
                    die!(~"Failed to open GLFW window");
                }
            };
        
        window.make_context_current();
        
        let mut done = false;
        
        while !done {
            glfw3::poll_events();
            if (window.get_key(glfw3::KEY_ESC) == glfw3::PRESS || window.get_param(glfw3::SHOULD_CLOSE) != 0) {
                done = true;
            }
        }
        
        window.destroy();
        glfw3::terminate();
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}