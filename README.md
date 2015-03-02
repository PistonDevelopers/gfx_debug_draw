# gfx_debug_draw

Library for batched renderering of lines and text in 3D space, using [gfx-rs](https://github.com/gfx-rs/gfx-rs).

## Usage

```rust
// Initializing...

let mut debug_renderer = DebugRendererBuilder::new(
	&mut graphics, // gfx::Graphics
	[viewport_width, viewport_height], // width, height of the SDL or GLFW frame/viewport
).build().ok().unwrap();

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

```

## Licenses of included assets

Default bitmap font was generated from [Google Noto](https://www.google.com/get/noto/) (Apache2)
