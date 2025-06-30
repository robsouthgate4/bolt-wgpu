use glfw::{fail_on_errors, Action, Context, Glfw, Key};

struct Bolt<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>
}

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

    let (mut window, events) = glfw.create_window(1024, 768, "Bolt", glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                },
                e => {
                    println!("{:?}", e);
                }
        }

        window.swap_buffers();
        }
    }
}
