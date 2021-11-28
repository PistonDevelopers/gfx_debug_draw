use gfx;
use gfx::handle::{DepthStencilView, RenderTargetView};
use gfx::traits::*;
use gfx::PipelineState;
use std::collections::hash_map::{Entry, HashMap};

use utils::grow_buffer;
use DebugRendererError;

pub struct LineRenderer<R: gfx::Resources> {
    vertex_data: Vec<Vertex>,
    vertex_buffer: gfx::handle::Buffer<R, Vertex>,
    pso_map: HashMap<gfx::format::Format, PipelineState<R, pipe::Meta>>,
    shaders: gfx::ShaderSet<R>,
}

impl<R: gfx::Resources> LineRenderer<R> {

    pub fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        initial_buffer_size: usize
    ) -> LineRenderer<R> {

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
        let vertex_buffer = factory.create_buffer(
            initial_buffer_size,
            gfx::buffer::Role::Vertex,
            gfx::memory::Usage::Dynamic,
            gfx::memory::Bind::empty()
        ).expect("Could not create vertex buffer");

        LineRenderer {
            vertex_data: Vec::new(),
            vertex_buffer: vertex_buffer,
            pso_map: HashMap::new(),
            shaders: set,
        }
    }

    fn prepare_pso<F: gfx::Factory<R>>(&mut self, factory: &mut F, format: gfx::format::Format) -> Result<(), gfx::PipelineStateError<String>> {
        Ok(if let Entry::Vacant(e) = self.pso_map.entry(format) {
            let init = pipe::Init {
                vbuf: (),
                u_model_view_proj: "u_model_view_proj",
                out_color: ("o_Color", format, gfx::state::ColorMask::all(), Some(gfx::preset::blend::ALPHA)),
                out_depth: gfx::preset::depth::LESS_EQUAL_WRITE,
            };
            let pso = factory.create_pipeline_state(
                &self.shaders,
                gfx::Primitive::LineList,
                gfx::state::Rasterizer::new_fill(),
                init
            )?;
            e.insert(pso);
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
    pub fn render<C: gfx::CommandBuffer<R>, F: gfx::Factory<R>, T: gfx::format::RenderFormat> (
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        factory: &mut F,
        color_target: &RenderTargetView<R, T>,
        depth_target: &DepthStencilView<R, gfx::format::DepthStencil>,
        projection: [[f32; 4]; 4],
    ) -> Result<(), DebugRendererError> {
        use gfx::memory::Typed;

        if self.vertex_data.len() > self.vertex_buffer.len() {
            self.vertex_buffer = grow_buffer(factory, &self.vertex_buffer, gfx::buffer::Role::Vertex, self.vertex_data.len());
        }

        encoder.update_buffer(&self.vertex_buffer, &self.vertex_data[..], 0)?;

        self.prepare_pso(factory, T::get_format())?;
        let pso = &self.pso_map[&T::get_format()];

        let data = pipe::Data {
            vbuf: self.vertex_buffer.clone(),
            u_model_view_proj: projection,
            out_color: color_target.raw().clone(),
            out_depth: depth_target.clone(),
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&self.vertex_buffer);
        encoder.draw(&slice, pso, &data);

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

gfx_pipeline_base!( pipe {
    vbuf: gfx::VertexBuffer<Vertex>,
    u_model_view_proj: gfx::Global<[[f32; 4]; 4]>,
    out_color: gfx::RawRenderTarget,
    out_depth: gfx::DepthTarget<::gfx::format::DepthStencil>,
});
