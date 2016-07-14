use gfx;
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx::traits::*;

use utils::{grow_buffer, MAT4_ID};

pub struct LineRenderer<R: gfx::Resources> {
    vertex_data: Vec<Vertex>,
    vertex_buffer: gfx::handle::Buffer<R, Vertex>,
    pso: gfx::PipelineState<R, pipe::Meta>,
}

impl<R: gfx::Resources> LineRenderer<R> {

    pub fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        initial_buffer_size: usize
    ) -> Result<LineRenderer<R>, gfx::PipelineStateError> {

        /* TODO: Update
        let vertex = gfx::ShaderSource {
            glsl_120: Some(VERTEX_SRC[0]),
            glsl_150: Some(VERTEX_SRC[1]),
            .. gfx::ShaderSource::empty()
        };
        */

        /* TODO: Update
        let fragment = gfx::ShaderSource {
            glsl_120: Some(FRAGMENT_SRC[0]),
            glsl_150: Some(FRAGMENT_SRC[1]),
            .. gfx::ShaderSource::empty()
        };
        */

        let set = factory.create_shader_set(&VERTEX_SRC[1], &FRAGMENT_SRC[1]).unwrap();
        let rasterizer = gfx::state::Rasterizer::new_fill();
        let pso = try!(factory.create_pipeline_state(
            &set, gfx::Primitive::LineList, rasterizer, pipe::new()
        ));

        let vertex_buffer = factory.create_buffer_dynamic(
            initial_buffer_size, gfx::BufferRole::Vertex, gfx::Bind::empty()
        ).expect("Could not create vertex buffer");

        Ok(LineRenderer {
            vertex_data: Vec::new(),
            vertex_buffer: vertex_buffer,
            pso: pso,
        })
    }

    ///
    /// Add a line to the batch to be drawn on 'render'
    ///
    pub fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        self.vertex_data.push(Vertex{position: start, color: color});
        self.vertex_data.push(Vertex{position: end, color: color});
    }

    ///
    /// Draw and clear the current batch of lines
    ///
    pub fn render<F: gfx::Factory<R>, C: gfx::CommandBuffer<R>> (
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        factory: &mut F,
        color_target: &RenderTargetView<R, gfx::format::Srgba8>,
        depth_target: &DepthStencilView<R, gfx::format::DepthStencil>,
        projection: [[f32; 4]; 4],
    ) -> Result<(), gfx::UpdateError<usize>> {

        if self.vertex_data.len() > self.vertex_buffer.len() {
            self.vertex_buffer = grow_buffer(factory, &self.vertex_buffer, gfx::BufferRole::Vertex, self.vertex_data.len());
        }

        try!(encoder.update_buffer(&self.vertex_buffer, &self.vertex_data[..], 0));

        /* TODO: Update
        self.params.model_view_proj = projection;
        */

        /* TODO: Update
        let mesh = gfx::Mesh::from_format(
            self.vertex_buffer.clone(),
            self.vertex_data.len() as gfx::VertexCount
        );
        */

        /* TODO: Update
        let slice = mesh.to_slice(gfx::PrimitiveType::Line);
        */

        /* TODO: Update
        stream.draw(
            &gfx::batch::bind(&self.state, &mesh, slice, &self.program, &self.params)
        ).unwrap();
        */

        let data = pipe::Data {
            vbuf: self.vertex_buffer.clone(),
            u_model_view_proj: projection,
            out_color: color_target.clone(),
            out_depth: depth_target.clone(),
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&self.vertex_buffer);
        encoder.draw(&slice, &self.pso, &data);

        self.vertex_data.clear();

        Ok(())
    }
}

static VERTEX_SRC: [&'static [u8]; 2] = [
b"
    #version 120

    uniform mat4 u_model_view_proj;
    attribute vec3 at_position;
    attribute vec4 at_color;
    varying vec4 v_color;

    void main() {
        gl_Position = u_model_view_proj * vec4(at_position, 1.0);
        v_color = at_color;
    }
",
b"
    #version 150 core

    uniform mat4 u_model_view_proj;
    in vec3 at_position;
    in vec4 at_color;
    out vec4 v_color;

    void main() {
        gl_Position = u_model_view_proj * vec4(at_position, 1.0);
        v_color = at_color;
    }
"];

static FRAGMENT_SRC: [&'static [u8]; 2] = [
b"
    #version 120

    varying vec4 v_color;

    void main() {
        gl_FragColor = v_color;
    }
",
b"
    #version 150

    in vec4 v_color;
    out vec4 out_color;

    void main() {
        out_color = v_color;
    }
"];

gfx_vertex_struct!( Vertex {
    position: [f32; 3] = "at_position",
    color: [f32; 4] = "at_color",
});

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]> = "u_model_view_proj",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "o_Color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});
