use bitmap_font::BitmapFont;
use line_renderer::LineRenderer;
use text_renderer::TextRenderer;
use image::{self, ImageFormat, DynamicImage};
use std::error::FromError;

use gfx::{
    Device,
    Frame,
    Graphics,
    ProgramError,
    TextureHandle,
};

use gfx_texture::{ Texture };

pub enum DebugRendererError {
    ShaderProgramError(ProgramError),
    BitmapFontTextureError,
}

impl FromError<ProgramError> for DebugRendererError {
    fn from_error(err: ProgramError) -> DebugRendererError {
        DebugRendererError::ShaderProgramError(err)
    }
}

pub struct DebugRendererBuilder<'a, D: Device>
        where D::Resources: 'a,
              D::Mapper: 'a,
              D::CommandBuffer: 'a,
              D: 'a {
    graphics: &'a mut Graphics<D>,
    frame_size: [u32; 2],
    initial_buffer_size: usize,
    bitmap_font: Option<BitmapFont>,
    bitmap_font_texture: Option<TextureHandle<D::Resources>>,
}

impl<'a, D: Device> DebugRendererBuilder<'a, D> {

    pub fn new(graphics: &'a mut Graphics<D>, frame_size: [u32; 2]) -> DebugRendererBuilder<D> {
        DebugRendererBuilder {
            graphics: graphics,
            frame_size: frame_size,
            initial_buffer_size: 64,
            bitmap_font: None,
            bitmap_font_texture: None,
        }
    }

    pub fn initial_buffer_size(&mut self, size: usize) -> &DebugRendererBuilder<D> {
        self.initial_buffer_size = size;
        self
    }

    pub fn bitmap_font(&mut self, bitmap_font: BitmapFont, bitmap_font_texture: TextureHandle<D::Resources>) -> &DebugRendererBuilder<D> {
        self.bitmap_font = Some(bitmap_font);
        self.bitmap_font_texture = Some(bitmap_font_texture);
        self
    }

    pub fn build(self) -> Result<DebugRenderer<D>, DebugRendererError> {

        let bitmap_font = match self.bitmap_font {
            Some(f) => f,
            None => BitmapFont::from_string(include_str!("../assets/notosans.fnt")).unwrap()
        };

        let bitmap_font_texture = match self.bitmap_font_texture {
            Some(t) => t,
            None => {
                if let DynamicImage::ImageRgba8(rgba_image) = image::load_from_memory_with_format(include_bytes!("../assets/notosans.png"), ImageFormat::PNG).unwrap() {
                    Texture::from_image(&mut self.graphics.device, &rgba_image).handle
                } else {
                    return Err(DebugRendererError::BitmapFontTextureError)
                }
            }
        };

        let line_renderer = try!(LineRenderer::new(self.graphics, self.initial_buffer_size));
        let text_renderer = try!(TextRenderer::new(self.graphics, self.frame_size, self.initial_buffer_size, bitmap_font, bitmap_font_texture));

        Ok(DebugRenderer {
            line_renderer: line_renderer,
            text_renderer: text_renderer,
        })
    }
}

pub struct DebugRenderer<D: Device> {
    line_renderer: LineRenderer<D>,
    text_renderer: TextRenderer<D>,
}

impl<D: Device> DebugRenderer<D> {

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
        graphics: &mut Graphics<D>,
        frame: &Frame<D::Resources>,
        projection: [[f32; 4]; 4],
    ) {
        self.line_renderer.render(graphics, frame, projection);
        self.text_renderer.render(graphics, frame, projection);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.text_renderer.resize(width, height);
    }
}
