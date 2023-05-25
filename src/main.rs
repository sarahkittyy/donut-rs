use glium::{self, Surface};

use glium::glutin as glu;
use glu::event::VirtualKeyCode as KeyCode;
use std::time;

use glam;

mod gfx;
mod camera;

use std::collections as col;

fn main() {
    // glium / glutin init
    let events_loop = glu::event_loop::EventLoop::new();
    let wb = glu::window::WindowBuilder::new()
        .with_inner_size(glu::dpi::LogicalSize::new(500, 500))
        .with_title("donut :3");
    let cb = glu::ContextBuilder::new()
        .with_multisampling(0)
        .with_pixel_format(24u8, 8u8)
        .with_srgb(false)
        .with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &events_loop).expect("Could not initialize glium display.");

    // app init
    let mut obj = gfx::load_mesh(&display, "assets/knot.obj").expect("failed to load mesh");
    let sprog = gfx::shaders(&display);

    let mut cam = camera::Camera::new();

    let proj = glam::Mat4::perspective_lh(75f32.to_radians(), 1.0, 0.1, 100.0);

    obj.pos = glam::vec3(0.0, 0.0, 25.0);
    cam.pos = glam::vec3(0.0, 10.0, 0.0);
    cam.yaw = 90.0;

    // for tracking frame times
    let mut t = time::Instant::now();

    let mut inputs = col::HashMap::<KeyCode, bool>::new();
    fn key_down(imap: &col::HashMap<KeyCode, bool>, k: KeyCode) -> bool {
        imap.get(&k).unwrap_or(&false).clone()
    }

    let mut clock = time::Duration::ZERO;

    let mut mouse_last_pos = glam::vec2(-1.0, -1.0);
    let mut mouse_diff = glam::vec2(0.0, 0.0);

    let mut focused = false;

    // main loop
    events_loop.run(move |ev, _, control_flow| {
        let wptr = display.gl_window();
        let win: &glu::window::Window = wptr.window();

        let now = time::Instant::now();
        let nft = now + time::Duration::from_millis(16);
        *control_flow = glu::event_loop::ControlFlow::WaitUntil(nft);

        let dt = now - t;
        t = now;
        clock += dt;

        // event handling
        match ev {
            glu::event::Event::WindowEvent { event, .. } => match event {
                glu::event::WindowEvent::CloseRequested => { *control_flow = glu::event_loop::ControlFlow::Exit; return; },
                glu::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        inputs.insert(key, input.state == glu::event::ElementState::Pressed);
                    }
                },
                glu::event::WindowEvent::CursorMoved { position, .. } => {
                    let cpos = glam::vec2(position.x as f32, position.y as f32);
                    if mouse_last_pos == glam::vec2(-1.0, -1.0) {
                        mouse_last_pos = cpos;
                    }
                    mouse_diff += cpos - mouse_last_pos;
                    mouse_last_pos = cpos;
                },
                glu::event::WindowEvent::Focused(f) => {
                    focused = f;
                },
                _ => return,
            },
            glu::event::Event::MainEventsCleared => {
                let mut step = || {
                    let vel = 12.0 / 60.0;
                    let mut lateral = glam::vec2(0.0, 0.0);
                    let mut vert: f32 = 0.0;

                    if key_down(&inputs, KeyCode::W) {
                        lateral.y += 1.0;
                    }
                    if key_down(&inputs, KeyCode::S) {
                        lateral.y -= 1.0;
                    }
                    if key_down(&inputs, KeyCode::A) {
                        lateral.x -= 1.0;
                    }
                    if key_down(&inputs, KeyCode::D) {
                        lateral.x += 1.0;
                    }
                    if key_down(&inputs, KeyCode::LShift) {
                        vert -= 1.0;
                    }
                    if key_down(&inputs, KeyCode::Space) {
                        vert += 1.0;
                    }
                    lateral = lateral.normalize_or_zero() * vel;
                    if lateral != glam::vec2(0.0, 0.0) { cam.lateral_move(lateral); }
                    if vert != 0.0 { cam.translate_y(vert * vel); }

                    // mouse update
                    //let sens = 0.05; // degrees per pixel
                    //cam.rotate_pitch(-mouse_diff.y * sens);
                    //cam.rotate_yaw(mouse_diff.x * sens);
    
                    let sens = 80.0 / 60.0;
                    if key_down(&inputs, KeyCode::I) {
                        cam.rotate_pitch(sens);
                    }
                    if key_down(&inputs, KeyCode::K) {
                        cam.rotate_pitch(-sens);
                    }
                    if key_down(&inputs, KeyCode::J) {
                        cam.rotate_yaw(sens);
                    }
                    if key_down(&inputs, KeyCode::L) {
                        cam.rotate_yaw(-sens);
                    }

                    mouse_diff = glam::vec2(0.0, 0.0); // reset mouse_diff (we've handled it)
                };
                // update
                let timestep = time::Duration::from_secs_f64(1.0 / 60.0);
                while clock >= timestep {
                    step();
                    clock -= timestep;
                }

                // drawing
                let dp = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let mut frame = display.draw();
                frame.clear_color_and_depth((0.2, 0.2, 0.8, 1.0), 1.0);
            
                let uniforms = glium::uniform! {
                    model: obj.model_matrix().to_cols_array_2d(),
                    view: glam::Mat4::from(cam).to_cols_array_2d(),
                    proj: proj.to_cols_array_2d(),
                    light_color: [1.0, 1.0, 1.0] as [f32; 3],
                    light_position: [5.0, 15.0, 5.0] as [f32; 3],
                    view_position: cam.pos.to_array(),
                };
                frame.draw(&obj.vbo, &obj.ibo, &sprog, &uniforms, &dp).expect("failed to draw mesh");

                frame.finish().expect("Could not finish frame.");
            },
            _ => (),
        }
    });
}
