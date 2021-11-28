use gfx;
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx::traits::*;
use gfx_text;
use vecmath::*;

use line_renderer::LineRenderer;

#[derive(Debug)]
pub enum DebugRendererError {
    PipelineStateError(gfx::PipelineStateError<String>),
    UpdateError(gfx::UpdateError<usize>),
    GfxTextError(gfx_text::Error)
}

impl From<gfx::PipelineStateError<String>> for DebugRendererError {
    fn from(err: gfx::PipelineStateError<String>) -> DebugRendererError {
        DebugRendererError::PipelineStateError(err)
    }
}

impl From<gfx::UpdateError<usize>> for DebugRendererError {
    fn from(err: gfx::UpdateError<usize>) -> DebugRendererError {
        DebugRendererError::UpdateError(err)
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
        let line_renderer = LineRenderer::new(&mut factory, initial_buffer_size);

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            text_renderer: text_renderer,
            factory: factory,
        })
    }

    pub fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        self.line_renderer.draw_line(start, end, color);
    }

    pub fn draw_marker(&mut self, position: [f32; 3], size: f32, color: [f32; 4]) {
        self.line_renderer.draw_line(vec3_add(position, [size, 0.0, 0.0]),
                                     vec3_add(position, [-size, 0.0, 0.0]),
                                     color);

        self.line_renderer.draw_line(vec3_add(position, [0.0, size, 0.0]),
                                     vec3_add(position, [0.0, -size, 0.0]),
                                     color);

        self.line_renderer.draw_line(vec3_add(position, [0.0, 0.0, size]),
                                     vec3_add(position, [0.0, 0.0, -size]),
                                     color);
    }

    pub fn draw_text_on_screen (
        &mut self,
        text: &str,
        screen_position: [i32; 2],
        color: [f32; 4],
    ) {
        self.text_renderer.add(text, screen_position, color);
    }

    pub fn draw_text_at_position (
        &mut self,
        text: &str,
        world_position: [f32; 3],
        color: [f32; 4],
    ) {
        self.text_renderer.add_at(text, world_position, color);
    }

    pub fn render<C: gfx::CommandBuffer<R>, T: gfx::format::RenderFormat>(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        color_target: &RenderTargetView<R, T>,
        depth_target: &DepthStencilView<R, gfx::format::DepthStencil>,
        projection: [[f32; 4]; 4],
    ) -> Result<(), DebugRendererError> {
        self.line_renderer.render(encoder, &mut self.factory,
            color_target, depth_target, projection)?;
        self.text_renderer.draw_at(encoder, color_target, projection)?;
        Ok(())
    }
}
