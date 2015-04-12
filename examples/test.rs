extern crate piston;
extern crate shader_version;
extern crate sdl2;
extern crate sdl2_window;
extern crate gfx;
extern crate camera_controllers;
extern crate vecmath;
extern crate env_logger;
extern crate gfx_debug_draw;
extern crate gfx_device_gl;

use gfx_debug_draw::DebugRenderer;

use std::cell::RefCell;
use std::rc::Rc;

use piston::window::{
    WindowSettings,
    OpenGLWindow,
    Window,
};

use piston::event::{
    Events,
    RenderEvent,
    ResizeEvent,
};

use vecmath::mat4_id;

use sdl2_window::Sdl2Window;

use camera_controllers::{
    OrbitZoomCamera,
    OrbitZoomCameraSettings,
    CameraPerspective,
    model_view_projection
};

use gfx::traits::*;

struct WindowOuput<R: gfx::Resources> {
    pub window: Rc<RefCell<Sdl2Window>>,
    frame: gfx::FrameBufferHandle<R>,
    mask: gfx::Mask,
    gamma: gfx::Gamma,
}

impl<R: gfx::Resources> gfx::Output<R> for WindowOuput<R> {

    fn get_handle(&self) -> Option<&gfx::FrameBufferHandle<R>> {
        Some(&self.frame)
    }

    fn get_size(&self) -> (gfx::tex::Size, gfx::tex::Size) {
        let piston::window::Size {width: w, height: h} = self.window.borrow().size();
        (w as gfx::tex::Size, h as gfx::tex::Size)
    }

    fn get_mask(&self) -> gfx::Mask {
        self.mask
    }

    fn get_gamma(&self) -> gfx::Gamma {
        self.gamma
    }
}

fn main() {

    env_logger::init().unwrap();

    let (win_width, win_height) = (640, 480);

    let mut window = Sdl2Window::new(
        shader_version::OpenGL::_3_2,
        WindowSettings::new(
            "Debug Render Test".to_string(),
            piston::window::Size { width: 640, height: 480 },
        ).exit_on_esc(true)
    );

    let mut graphics = gfx_device_gl::create(|s| window.get_proc_address(s)).into_graphics();

    let window = Rc::new(RefCell::new(window));

    let window_output = WindowOuput {
        window: window.clone(),
        frame: graphics.factory.get_main_frame_buffer(),
        mask: gfx::COLOR | gfx::DEPTH | gfx::STENCIL,
        gamma: gfx::Gamma::Original
    };

    let clear = gfx::ClearData {
        color: [0.3, 0.3, 0.3, 1.0],
        depth: 1.0,
        stencil: 0
    };

    let mut debug_renderer = DebugRenderer::new(&mut graphics, [win_width as u32, win_height as u32], 64, None, None).ok().unwrap();

    let model = mat4_id();
    let mut projection = CameraPerspective {
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

    for e in window.events() {

        e.resize(|width, height| {
            debug_renderer.resize(width, height);

            // Update projection matrix
            projection = CameraPerspective {
                fov: 90.0f32,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: (width as f32) / (height as f32)
            }.projection();
        });

        orbit_zoom_camera.event(&e);

        if let Some(args) = e.render_args() {
            graphics.clear(clear, gfx::COLOR | gfx::DEPTH, &window_output);

            let camera_projection = model_view_projection(
                model,
                orbit_zoom_camera.camera(args.ext_dt).orthogonal(),
                projection
            );

            // Draw axes
            debug_renderer.draw_line([0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
            debug_renderer.draw_line([0.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 1.0, 0.0, 1.0]);
            debug_renderer.draw_line([0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [0.0, 0.0, 1.0, 1.0]);

            debug_renderer.draw_text_on_screen(&format!("FPS: {}", 1.0 / args.ext_dt)[..], [10, 10], [1.0, 0.4, 0.4, 0.7]);

            debug_renderer.draw_text_at_position(
                "X",
                [6.0, 0.0, 0.0],
                [1.0, 0.0, 0.0, 1.0],
            );

            debug_renderer.draw_text_at_position(
                "Y",
                [0.0, 6.0, 0.0],
                [0.0, 1.0, 0.0, 1.0],
            );

            debug_renderer.draw_text_at_position(
                "Z",
                [0.0, 0.0, 6.0],
                [0.0, 0.0, 1.0, 1.0],
            );

            debug_renderer.render(&mut graphics, &window_output, camera_projection);

            graphics.end_frame();
        }
    }
}
