extern mod glfw3;

fn main() {
    
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn {
        
        if !glfw3::init() {
            glfw3::terminate();
            fail(~"Failed to initialize GLFW\n");
        }
        
        let mut window = glfw3::Window::create(300, 300, "Hello, I am a window.", glfw3::Windowed);
        
        io::println(fmt!("Window ptr: %d", window.ptr as int));
        
        if window.ptr.is_null() {
            glfw3::terminate();
            fail(~"Failed to open GLFW window");
        }
        
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