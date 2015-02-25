use std::default::Default;

use gfx::{
    BufferUsage,
    Device,
    DeviceExt,
    DrawState,
    Frame,
    GlDevice,
    GlResources,
    Graphics,
    Mesh,
    PrimitiveType,
    ProgramError,
    ProgramHandle,
    BufferHandle,
    ToSlice,
    Slice,
    SliceKind,
    VertexCount,
    BlendPreset,
};

use gfx::tex::{SamplerInfo, FilterMethod, WrapMode};

use gfx::batch::{
    BatchError,
    RefBatch,
};

use gfx::shade::TextureParam;

use gfx_texture::{ Texture };

use bitmap_font::{BitmapFont, BitmapCharacter};

pub struct TextRenderer {
    program: ProgramHandle<GlResources>,
    state: DrawState,
    bitmap_font: BitmapFont,
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    vertex_buffer: BufferHandle<GlResources, Vertex>,
    vertex_buffer_size: usize,
    index_buffer: BufferHandle<GlResources, u32>,
    index_buffer_size: usize,
    params: TextShaderParams,
}

// TODO Allow optional font override?

impl TextRenderer {


    pub fn new(graphics: &mut Graphics<GlDevice>, initial_buffer_size: usize) -> Result<TextRenderer, ProgramError> {

        let program = match graphics.device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone()) {
            Ok(program_handle) => program_handle,
            Err(e) => return Err(e),
        };

        let vertex_buffer = graphics.device.create_buffer::<Vertex>(initial_buffer_size, BufferUsage::Dynamic);

        let index_buffer = graphics.device.create_buffer::<u32>(1024, BufferUsage::Dynamic);

        let font_texture = Texture::from_path(&mut graphics.device, &Path::new("assets/font.png")).unwrap();
        let bitmap_font = BitmapFont::from_path(&Path::new("assets/font.fnt")).unwrap();

        let sampler = graphics.device.create_sampler(
           SamplerInfo::new(
               FilterMethod::Scale,
               WrapMode::Clamp
            )
        );

        let state = DrawState::new().blend(BlendPreset::Alpha);

        Ok(TextRenderer {
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            bitmap_font: bitmap_font,
            program: program,
            state: state,
            vertex_buffer: vertex_buffer,
            vertex_buffer_size: initial_buffer_size,
            index_buffer: index_buffer,
            index_buffer_size: 1024,
            params: TextShaderParams {
                u_model_view_proj: MAT4_ID,
                u_screen_size: [640.0, 480.0],
                u_tex_font: (font_texture.handle, Some(sampler)),
            },
        })
    }

    pub fn draw_text_on_screen(
        &mut self,
        text: &str,
        screen_position: [i32; 2],
        color: [f32; 4],
    ) {
        let [mut x, mut y] = screen_position;

        let scale_w = self.bitmap_font.scale_w as f32;
        let scale_h = self.bitmap_font.scale_h as f32;

        // placeholder for characters missing from font
        let default_character = Default::default();

        for character in text.chars() {

            let bc = match self.bitmap_font.characters.get(&character) {
                Some(c) => c,
                None => &default_character,
            };

            // Push quad vertices in CCW direction
            let index = self.vertex_data.len();

            let x_offset = (bc.xoffset as i32 + x) as f32;
            let y_offset = (bc.yoffset as i32 + y) as f32;


            // 0 - top left
            self.vertex_data.push(Vertex{
                position: [
                    x_offset,
                    y_offset,
                ],
                color: color,
                texcoords: [
                    bc.x as f32 / scale_w,
                    bc.y as f32 / scale_h,
                ],
            });

            // 1 - bottom left
            self.vertex_data.push(Vertex{
                position: [
                    x_offset,
                    bc.height as f32 + y_offset
                ],
                color: color,
                texcoords: [
                    bc.x as f32 / scale_w,
                    (bc.y + bc.height) as f32 / scale_h,
                ],
            });

            // 2 - bottom right
            self.vertex_data.push(Vertex{
                position: [
                    bc.width as f32 + x_offset,
                    bc.height as f32 + y_offset,
                ],
                color: color,
                texcoords: [
                    (bc.x + bc.width) as f32 / scale_w,
                    (bc.y + bc.height) as f32 / scale_h,
                ],
            });


            // 3 - top right
            self.vertex_data.push(Vertex{
                position: [
                    bc.width as f32 + x_offset,
                    y_offset,
                ],
                color: color,
                texcoords: [
                    (bc.x + bc.width) as f32 / scale_w,
                    bc.y as f32 / scale_h,
                ],
            });


            // Top-left triangle
            self.index_data.push((index + 0) as u32);
            self.index_data.push((index + 1) as u32);
            self.index_data.push((index + 3) as u32);

            // Bottom-right triangle
            self.index_data.push((index + 3) as u32);
            self.index_data.push((index + 1) as u32);
            self.index_data.push((index + 2) as u32);

            x += bc.xadvance as i32;
        }
    }

    ///
    /// Draw and clear the current batch of lines
    ///
    pub fn render(
        &mut self,
        graphics: &mut Graphics<GlDevice>,
        frame: &Frame<GlResources>,
        projection: [[f32; 4]; 4],
    ) {
        // self.draw_test_quad();

        self.params.u_model_view_proj = projection;

        self.grow_vertex_buffer(graphics);

        graphics.device.update_buffer(self.vertex_buffer.clone(), &self.vertex_data[..], 0);
        graphics.device.update_buffer(self.index_buffer.clone(), &self.index_data[..], 0);

        match self.make_batch(graphics) {
            Ok(batch) =>  {
                graphics.draw(&batch, &self.params, frame);
            },
            Err(e) => {
                println!("Error creating debug render batch: {:?}", e);
            },
        }

        self.vertex_data.clear();
        self.index_data.clear();
    }

    ///
    /// Grow the vertex buffer if necessary
    ///
    fn grow_vertex_buffer(&mut self, graphics: &mut Graphics<GlDevice>) {

        let required_size = self.vertex_data.len();

        if required_size > self.vertex_buffer_size {
            graphics.device.delete_buffer(self.vertex_buffer);

            while self.vertex_buffer_size < required_size {
                self.vertex_buffer_size *= 2;
            }

            self.vertex_buffer = graphics.device.create_buffer::<Vertex>(self.vertex_buffer_size, BufferUsage::Dynamic);
        }
    }

    ///
    /// Construct a new ref batch for the current number of vertices
    ///
    fn make_batch(&mut self, graphics: &mut Graphics<GlDevice>) -> Result<RefBatch<TextShaderParams>, BatchError> {
        let mesh = Mesh::from_format(
            self.vertex_buffer.clone(),
            self.vertex_data.len() as VertexCount
        );

        let slice = Slice {
            start: 0,
            end: self.index_data.len() as u32,
            prim_type: PrimitiveType::TriangleList,
            kind: SliceKind::Index32(self.index_buffer.clone(), 0),
        };

        graphics.make_batch(&self.program, &mesh, slice, &self.state)
    }
}

static VERTEX_SRC: &'static [u8] = b"
    #version 150 core

    uniform vec2 u_screen_size;

    in vec2 position;
    in vec4 color;
    in vec2 texcoords;
    out vec4 v_color;
    out vec2 v_TexCoord;

    void main() {
        float x = 2 * position.x / u_screen_size.x - 1;
        float y = 1 - 2 * position.y / u_screen_size.y;
        gl_Position = vec4(x, y, 0.5, 1.0);
        v_TexCoord = texcoords;
        v_color = color;
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 150

    uniform sampler2D u_tex_font;

    in vec4 v_color;
    in vec2 v_TexCoord;
    out vec4 out_color;

    void main() {
        vec4 font_color = texture(u_tex_font, v_TexCoord);
        out_color = vec4(v_color.xyz, font_color.a * v_color.a);
    }
";

#[vertex_format]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Vertex {
    position: [f32; 2], // in Pixels relative to top-left of screen
    texcoords: [f32; 2],
    color: [f32; 4],
}

#[shader_param]
struct TextShaderParams {
    u_model_view_proj: [[f32; 4]; 4],
    u_screen_size: [f32; 2],
    u_tex_font: TextureParam<GlResources>,
}

static MAT4_ID: [[f32; 4]; 4] =
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];
