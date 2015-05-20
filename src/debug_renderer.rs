use line_renderer::LineRenderer;

use gfx;
use gfx::traits::*;
use gfx_text;

#[derive(Debug)]
pub enum DebugRendererError {
    ShaderProgramError(gfx::ProgramError),
    BufferUpdateError(gfx::device::BufferUpdateError),
    GfxTextError(gfx_text::Error)
}

impl From<gfx::ProgramError> for DebugRendererError {
    fn from(err: gfx::ProgramError) -> DebugRendererError {
        DebugRendererError::ShaderProgramError(err)
    }
}

impl From<gfx::device::BufferUpdateError> for DebugRendererError {
    fn from(err: gfx::device::BufferUpdateError) -> DebugRendererError {
        DebugRendererError::BufferUpdateError(err)
    }
}

impl From<gfx_text::Error> for DebugRendererError {
    fn from(err: gfx_text::Error) -> DebugRendererError {
        DebugRendererError::GfxTextError(err)
    }
}

pub struct DebugRenderer<R: gfx::Resources, F: Factory<R>> {
    line_renderer: LineRenderer<R>,
    text_renderer: gfx_text::Renderer<R, F>,
    factory: F,
}

impl<R: gfx::Resources, F: Factory<R>> DebugRenderer<R, F> {

    pub fn new (
        factory: F,
        text_renderer: gfx_text::Renderer<R, F>,
        initial_buffer_size: usize,
    ) -> Result<DebugRenderer<R, F>, DebugRendererError> {

        let mut factory = factory;
        let line_renderer = try!(LineRenderer::new(&mut factory, initial_buffer_size));

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            text_renderer: text_renderer,
            factory: factory,
        })
    }

    pub fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        self.line_renderer.draw_line(start, end, color);
    }

    pub fn draw_text_on_screen (
        &mut self,
        text: &str,
        screen_position: [i32; 2],
        color: [f32; 4],
    ) {
        self.text_renderer.draw(text, screen_position, color);
    }

    pub fn draw_text_at_position (
        &mut self,
        text: &str,
        world_position: [f32; 3],
        color: [f32; 4],
    ) {
        self.text_renderer.draw_at(text, world_position, color);
    }

    pub fn render<S: gfx::Stream<R>> (
        &mut self,
        stream: &mut S,
        projection: [[f32; 4]; 4],
    ) -> Result<(), DebugRendererError> {
        try!(self.line_renderer.render(stream, &mut self.factory, projection));
        try!(self.text_renderer.draw_end_at(stream, projection));
        Ok(())
    }
}
