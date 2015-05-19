# gfx_debug_draw [![Build Status](https://travis-ci.org/PistonDevelopers/gfx-debug-draw.png?branch=master)](https://travis-ci.org/PistonDevelopers/gfx-debug-draw)

Library for batched renderering of lines and text in 3D space, using [gfx-rs](https://github.com/gfx-rs/gfx-rs).

[Documentation](http://www.piston.rs/docs/gfx-debug-draw/gfx_debug_draw/)

## Usage

```rust
// Initializing...

let mut debug_renderer = DebugRenderer::new(
	factory // a gfx::Factory, to be owned by DebugRenderer
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

// Render the final batch of all lines and text currently present in the vertex/index buffers

debug_renderer.render(
	stream, // &mut gfx::Stream
	camera_projection, // Current camera projection matrix
);

```
