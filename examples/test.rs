#![feature(plugin)]
#![plugin(gfx_macros)]

extern crate piston;
extern crate shader_version;
extern crate sdl2;
extern crate sdl2_window;
extern crate gfx;
extern crate cam;
extern crate vecmath;
extern crate env_logger;
extern crate "gfx_gl" as gl;
extern crate "gfx-debug-draw" as gfx_debug_draw;

use gfx_debug_draw::{DebugRenderer};

#[macro_use]
extern crate gfx_macros;

use std::old_path::posix::Path;

use gl::Gl;
use gl::types::*;

use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::event::{
    events,
    RenderEvent,
};

use std::default::Default;
use vecmath::mat4_id;

use gfx::{ Device, DeviceExt, ToSlice };

use sdl2_window::Sdl2Window;

use std::old_io as io;
use std::old_io::{File, BufferedReader};

use cam::{
    OrbitZoomCamera,
    OrbitZoomCameraSettings,
    CameraPerspective,
    model_view_projection
};

fn main() {

    env_logger::init().unwrap();

    let (win_width, win_height) = (640, 480);

    let mut window = Sdl2Window::new(
        shader_version::OpenGL::_3_2,
        WindowSettings {
            title: "Debug Render Test".to_string(),
            size: [640, 480],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4
        }
    );

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    let frame = gfx::Frame::new(win_width as u16, win_height as u16);

    let window = RefCell::new(window);

    let clear = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0
    };

    let mut graphics = gfx::Graphics::new(device);

    let mut debug_renderer = DebugRenderer::new(&mut graphics, 1).unwrap();

    let model = mat4_id();
    let projection = CameraPerspective {
        fov: 90.0f32,
        near_clip: 0.1,
        far_clip: 1000.0,
        aspect_ratio: (win_width as f32) / (win_height as f32)
    }.projection();

    let mut orbit_zoom_camera: OrbitZoomCamera<f32> = OrbitZoomCamera::new(
        [0.0, 0.0, 0.0],
        OrbitZoomCameraSettings::default()
    );

    // Start event loop

    let gl = Gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    for e in events(&window) {

        orbit_zoom_camera.event(&e);

        if let Some(args) = e.render_args() {
            graphics.clear(clear, gfx::COLOR | gfx::DEPTH, &frame);

            // Draw axes
            debug_renderer.draw_line([0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
            debug_renderer.draw_line([0.0, 0.0, 0.0], [0.0, 10.0, 0.0], [0.0, 1.0, 0.0, 1.0]);
            debug_renderer.draw_line([0.0, 0.0, 0.0], [0.0, 0.0, 10.0], [0.0, 0.0, 1.0, 1.0]);

            debug_renderer.draw_text_on_screen("Hello world!", [10, 10], [1.0, 0.4, 0.4, 0.7]);

            let camera_projection = model_view_projection(
                model,
                orbit_zoom_camera.camera(args.ext_dt).orthogonal(),
                projection
            );

            debug_renderer.render(&mut graphics, &frame, camera_projection);

            graphics.end_frame();
        }
    }
}
