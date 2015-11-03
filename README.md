# gfx_debug_draw [![Build Status](https://travis-ci.org/PistonDevelopers/gfx_debug_draw.png?branch=master)](https://travis-ci.org/PistonDevelopers/gfx-debug-draw)

Library for batched renderering of lines and text in 3D space, using [gfx-rs](https://github.com/gfx-rs/gfx-rs).

[Documentation](http://www.piston.rs/docs/gfx-debug-draw/gfx_debug_draw/)

## Usage

```rust
// Initializing...

// Create gfx_text::Renderer to be used by the DebugRenderer
let text_renderer = {
    let factory = piston_window.device.borrow_mut().spawn_factory(); // gfx::Factory
    gfx_text::new(factory).unwrap() // can optionally configure text renderer here (font, color)
};

let mut debug_renderer = DebugRenderer::new(
    piston_window.device.borrow_mut().spawn_factory(), // gfx::Factory
    text_renderer,
	64, // Initial size of vertex buffers
).ok().unwrap();

...

// In render loop...

// Draw red line from origin along x-axis
debug_renderer.draw_line(
	[0.0, 0.0, 0.0], // Start position
	[5.0, 0.0, 0.0], // End position
	[1.0, 0.0, 0.0, 1.0], // Line color
);

// Draw an 'X' on the x-axis, at the end of the line drawn above.
debug_renderer.draw_text_at_position(
	"X", // String to draw
	[6.0, 0.0, 0.0], // World-space position to draw at
	[1.0, 0.0, 0.0, 1.0], // Text color
);

// Draw salmoney-colored text 10 pixels down and right from the top left corner of the screen
debug_renderer.draw_text_on_screen(
	"Hello World!", // Text to draw
	[10, 10], // Pixel coordinates relative to top-left corner of screen
	[1.0, 0.4, 0.4, 0.7] // Text color
);

// Draw a yellow position marker
debug_renderer.draw_marker(
    [1.0, 2.0, 3.0],  // Position
    0.5, // Size
    [1.0, 1.0, 0.0, 1.0] // Color
);

// Render the final batch of all lines and text currently present in the vertex/index buffers

debug_renderer.render(
	stream, // &mut gfx::Stream
	camera_projection, // Current camera projection matrix
);

```

Draw commands can also be queued up with static methods, which is useful when you want to debug
something in a context where you have no access to the DebugRenderer instance.

```rust
fn foobar() {
   ...
   let x: Vector3<f32> = some_expression;
   // Visually debug the value of `x` with a red position marker:
   gfx_debug_draw::draw_marker(x, 1.0, [1.0, 0.0, 0.0, 1.0]);
   ...
}
```
