extern crate shader_version;
extern crate sdl2_window;
extern crate gfx_text;
extern crate camera_controllers;
extern crate vecmath;
extern crate gfx_debug_draw;
extern crate gfx_device_gl;
extern crate piston_window;
extern crate current;

use gfx_debug_draw::DebugRenderer;

use vecmath::mat4_id;

use sdl2_window::Sdl2Window;
use piston_window::*;

use camera_controllers::{
    OrbitZoomCamera,
    OrbitZoomCameraSettings,
    CameraPerspective,
    model_view_projection
};

use current::{Current, CurrentGuard};

fn main() {

    let (win_width, win_height) = (640, 480);

    let mut piston_window: PistonWindow<Sdl2Window> =
        WindowSettings::new("Debug Render Test", (win_width, win_height))
         .exit_on_esc(true)
         .graphics_api(shader_version::OpenGL::V3_2)
         .build()
         .unwrap();

    let mut debug_renderer = {
        let text_renderer = {
            gfx_text::new(piston_window.factory.clone()).unwrap()
        };
        DebugRenderer::new(piston_window.factory.clone(), text_renderer, 64).ok().unwrap()
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

    while let Some(e) = piston_window.next() {

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

        piston_window.draw_3d(&e, |window| {
            let args = e.render_args().unwrap();

            window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
            window.encoder.clear_depth(&window.output_stencil, 1.0);

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

            {
                let guard = CurrentGuard::new(&mut debug_renderer);
                draw_stuff();
                drop(guard);
            }

            if let Err(e) = debug_renderer.render(
                &mut window.encoder,
                &window.output_color,
                &window.output_stencil,
                camera_projection
            ) {
                println!("{:?}", e);
            }
        });
    }
}

// To access the current debug renderer we need a concrete type.
type GlDebugRenderer = DebugRenderer<gfx_device_gl::Resources,
                                     gfx_device_gl::Factory>;

fn draw_stuff() {
    let debug_renderer = unsafe { &mut *Current::<GlDebugRenderer>::new() };

    debug_renderer.draw_line([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]);
    debug_renderer.draw_text_on_screen("Stuff", [10, 10], [0.0, 0.0, 1.0, 1.0]);
    debug_renderer.draw_text_at_position("Things", [2.0, 2.0, 2.0], [1.0, 1.0, 1.0, 1.0]);
    debug_renderer.draw_marker([-2.0, -2.0, -2.0], 0.5, [1.0, 1.0, 0.0, 1.0]);
}
