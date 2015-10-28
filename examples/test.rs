extern crate piston;
extern crate shader_version;
extern crate sdl2;
extern crate sdl2_window;
extern crate gfx;
extern crate gfx_text;
extern crate camera_controllers;
extern crate vecmath;
extern crate gfx_debug_draw;
extern crate gfx_device_gl;
extern crate piston_window;

use gfx_debug_draw::DebugRenderer;

use std::cell::RefCell;
use std::rc::Rc;

use piston::window::WindowSettings;

use piston::input::{
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

use gfx::traits::Stream;

fn main() {

    let (win_width, win_height) = (640, 480);

    let window: Sdl2Window = WindowSettings::new(
            "Debug Render Test".to_string(),
            piston::window::Size { width: 640, height: 480 },
        ).exit_on_esc(true)
         .opengl(shader_version::OpenGL::V3_2)
         .build()
         .unwrap();
    let window = Rc::new(RefCell::new(window));

    let piston_window = piston_window::PistonWindow::new(window, piston_window::empty_app());

    let mut debug_renderer = {
        let text_renderer = {
            gfx_text::new(piston_window.factory.borrow().clone()).unwrap()
        };
        DebugRenderer::new(piston_window.factory.borrow().clone(), text_renderer, 64).ok().unwrap()
    };

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

    for e in piston_window {

        e.resize(|width, height| {
            // Update projection matrix
            projection = CameraPerspective {
                fov: 90.0f32,
                near_clip: 0.1,
                far_clip: 1000.0,
                aspect_ratio: (width as f32) / (height as f32)
            }.projection();
        });

        orbit_zoom_camera.event(&e);

        e.draw_3d(|stream| {
            let args = e.render_args().unwrap();

            stream.clear(gfx::ClearData {
                color: [0.3, 0.3, 0.3, 1.0],
                depth: 1.0,
                stencil: 0,
            });

            let camera_projection = model_view_projection(
                model,
                orbit_zoom_camera.camera(args.ext_dt).orthogonal(),
                projection
            );

            // Draw axes
            debug_renderer.draw_line([0.0, 0.0, 0.0], [5.0, 0.0, 0.0], [1.0, 0.0, 0.0, 1.0]);
            debug_renderer.draw_line([0.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 1.0, 0.0, 1.0]);
            debug_renderer.draw_line([0.0, 0.0, 0.0], [0.0, 0.0, 5.0], [0.0, 0.0, 1.0, 1.0]);

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

            debug_renderer.draw_marker(
                [5.0, 5.0, 5.0],
                1.0,
                [0.0, 0.0, 1.0, 1.0],
            );

            debug_renderer.draw_line([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]);
            debug_renderer.draw_text_on_screen("Stuff", [10, 10], [0.0, 0.0, 1.0, 1.0]);
            debug_renderer.draw_text_at_position("Things", [2.0, 2.0, 2.0], [1.0, 1.0, 1.0, 1.0]);
            debug_renderer.draw_marker([-2.0, -2.0, -2.0], 0.5, [1.0, 1.0, 0.0, 1.0]);

            if let Err(e) = debug_renderer.render(stream, camera_projection) {
                println!("{:?}", e);
            }
        });
    }
}
