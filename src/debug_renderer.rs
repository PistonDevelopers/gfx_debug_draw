use bitmap_font::BitmapFont;
use line_renderer::LineRenderer;

use gfx;
use gfx::traits::*;
use gfx_texture;
use image;

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
    // text_renderer: TextRenderer<R>,
}

impl<R: gfx::Resources> DebugRenderer<R> {

    pub fn new<F: Factory<R>, S: Stream<R>> (
        factory: &mut F,
        stream: &mut S,
        initial_buffer_size: usize,
        bitmap_font: Option<BitmapFont>,
        bitmap_font_texture: Option<gfx::handle::Texture<R>>,
    ) -> Result<DebugRenderer<R>, DebugRendererError> {

        let (w, h) = stream.get_output().get_size();

        let bitmap_font = match bitmap_font {
            Some(f) => f,
            None => BitmapFont::from_string(include_str!("../assets/notosans.fnt")).unwrap()
        };

        let bitmap_font_texture = match bitmap_font_texture {
            Some(t) => t,
            None => {
                if let image::DynamicImage::ImageRgba8(rgba_image) = image::load_from_memory_with_format(include_bytes!("../assets/notosans.png"), image::ImageFormat::PNG).unwrap() {
                    gfx_texture::Texture::from_image(factory, &rgba_image, false, false, false).handle()
                } else {
                    return Err(DebugRendererError::BitmapFontTextureError)
                }
            }
        };

        let line_renderer = try!(LineRenderer::new(factory, initial_buffer_size));

        // let text_renderer = try!(TextRenderer::new(factory, frame_size, initial_buffer_size, bitmap_font, bitmap_font_texture));

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            // text_renderer: text_renderer,
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
        // self.text_renderer.draw_text_on_screen(text, screen_position, color);
    }

    pub fn draw_text_at_position (
        &mut self,
        text: &str,
        world_position: [f32; 3],
        color: [f32; 4],
    ) {
        // self.text_renderer.draw_text_at_position(text, world_position, color);
    }

    pub fn render<S: gfx::Stream<R>, F: Factory<R>> (
        &mut self,
        stream: &mut S,
        factory: &mut F,
        projection: [[f32; 4]; 4],
    ) {
        self.line_renderer.render(stream, factory, projection);
        // self.text_renderer.render(stream, factory, projection);
    }
}
