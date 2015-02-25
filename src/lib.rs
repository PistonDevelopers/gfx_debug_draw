#![feature(plugin)]
#![feature(old_io)]
#![feature(old_path)]
#![plugin(gfx_macros)]

extern crate gfx;
extern crate gfx_texture;
extern crate xml;
extern crate vecmath;

mod line_renderer;
mod text_renderer;
mod bitmap_font;

use line_renderer::LineRenderer;
use text_renderer::TextRenderer;

use gfx::{
    Frame,
    GlDevice,
    GlResources,
    Graphics,
    ProgramError,
};

pub struct DebugRenderer {
    line_renderer: LineRenderer,
    text_renderer: TextRenderer,
}

impl DebugRenderer {

    pub fn new(
        graphics: &mut Graphics<GlDevice>,
        frame_size: [u32; 2],
        initial_buffer_size: usize,
        font_xml_path: &Path,
        font_texture_path: &Path,
    ) -> Result<DebugRenderer, ProgramError> {

        let line_renderer = try!(LineRenderer::new(graphics, initial_buffer_size));
        let text_renderer = try!(TextRenderer::new(graphics, frame_size, initial_buffer_size, font_xml_path, font_texture_path));

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            text_renderer: text_renderer,
        })
    }

    pub fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        self.line_renderer.draw_line(start, end, color);
    }

    pub fn draw_text_on_screen(
        &mut self,
        text: &str,
        screen_position: [i32; 2],
        color: [f32; 4],
    ) {
        self.text_renderer.draw_text_on_screen(text, screen_position, color);
    }

    pub fn draw_text_at_position(
        &mut self,
        text: &str,
        world_position: [f32; 3],
        color: [f32; 4],
    ) {
        self.text_renderer.draw_text_at_position(text, world_position, color);
    }

    pub fn render(
        &mut self,
        graphics: &mut Graphics<GlDevice>,
        frame: &Frame<GlResources>,
        projection: [[f32; 4]; 4],
    ) {
        self.line_renderer.render(graphics, frame, projection);
        self.text_renderer.render(graphics, frame, projection);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.text_renderer.resize(width, height);
    }
}
