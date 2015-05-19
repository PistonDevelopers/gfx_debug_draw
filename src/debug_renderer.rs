use line_renderer::LineRenderer;

use gfx;
use gfx::traits::*;
use gfx_text;

#[derive(Debug)]
pub enum DebugRendererError {
    ShaderProgramError(gfx::ProgramError),
    BitmapFontTextureError,
}

impl From<gfx::ProgramError> for DebugRendererError {
    fn from(err: gfx::ProgramError) -> DebugRendererError {
        DebugRendererError::ShaderProgramError(err)
    }
}

pub struct DebugRenderer<R: gfx::Resources> {
    line_renderer: LineRenderer<R>,
    text_renderer: gfx_text::Renderer<R>,
}

impl<R: gfx::Resources> DebugRenderer<R> {

    pub fn new<F: Factory<R>> (
        factory: &mut F,
        initial_buffer_size: usize,
    ) -> Result<DebugRenderer<R>, DebugRendererError> {

        let line_renderer = try!(LineRenderer::new(factory, initial_buffer_size));
        let text_renderer = gfx_text::new(factory).unwrap();

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            text_renderer: text_renderer,
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

    pub fn render<S: gfx::Stream<R>, F: Factory<R>> (
        &mut self,
        stream: &mut S,
        factory: &mut F,
        projection: [[f32; 4]; 4],
    ) {
        self.line_renderer.render(stream, factory, projection);
        self.text_renderer.draw_end_at(factory, stream, projection);
    }
}
