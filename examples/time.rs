extern mod std;
extern mod glfw3;

fn main() {
    
    if !glfw3::init() {
        glfw3::terminate();
        fail(~"Failed to initialize GLFW");
    }
    
    let mut time = 0f64;
    glfw3::set_time(time);
    
    for 40.times {
        let newTime = glfw3::get_time();
        let delta = newTime - time;
        time = newTime;
        io::println(fmt!("dt: %?", delta));
    }
    
    glfw3::terminate();
}
