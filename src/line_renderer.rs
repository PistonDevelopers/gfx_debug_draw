use std::marker::PhantomData;

use gfx;
use gfx::traits::*;

use utils::{grow_buffer, MAT4_ID};

pub struct LineRenderer<R: gfx::Resources> {
    program: gfx::handle::Program<R>,
    state: gfx::DrawState,
    vertex_data: Vec<Vertex>,
    vertex_buffer: gfx::handle::Buffer<R, Vertex>,
    params: LineShaderParams<R>,
}

impl<R: gfx::Resources> LineRenderer<R> {

    pub fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        initial_buffer_size: usize
    ) -> Result<LineRenderer<R>, gfx::ProgramError> {

        let vertex = gfx::ShaderSource {
            glsl_120: Some(VERTEX_SRC[0]),
            glsl_150: Some(VERTEX_SRC[1]),
            .. gfx::ShaderSource::empty()
        };

        let fragment = gfx::ShaderSource {
            glsl_120: Some(FRAGMENT_SRC[0]),
            glsl_150: Some(FRAGMENT_SRC[1]),
            .. gfx::ShaderSource::empty()
        };

        let program = match factory.link_program_source(vertex, fragment){
            Ok(program_handle) => program_handle,
            Err(e) => return Err(e),
        };

        let vertex_buffer = factory.create_buffer_dynamic(initial_buffer_size, gfx::BufferRole::Vertex);

        Ok(LineRenderer {
            vertex_data: Vec::new(),
            program: program,
            state: gfx::DrawState::new(),
            vertex_buffer: vertex_buffer,
            params: LineShaderParams {
                model_view_proj: MAT4_ID,
                _r: PhantomData,
            },
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
    pub fn render<
        S: gfx::Stream<R>,
        F: gfx::Factory<R>,
    > (
        &mut self,
        stream: &mut S,
        factory: &mut F,
        projection: [[f32; 4]; 4],
    ) {

        if self.vertex_data.len() > self.vertex_buffer.len() {
            self.vertex_buffer = grow_buffer(factory, &self.vertex_buffer, gfx::BufferRole::Vertex, self.vertex_data.len());
        }

        factory.update_buffer(&self.vertex_buffer, &self.vertex_data[..], 0);


        self.params.model_view_proj = projection;

        let mesh = gfx::Mesh::from_format(
            self.vertex_buffer.clone(),
            self.vertex_data.len() as gfx::VertexCount
        );

        let slice = mesh.to_slice(gfx::PrimitiveType::Line);

        stream.draw(
            &gfx::batch::bind(&self.state, &mesh, slice, &self.program, &self.params)
        ).unwrap();

        self.vertex_data.clear();
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

gfx_vertex!( Vertex {
    at_position@ position: [f32; 3],
    at_color@ color: [f32; 4],
});

gfx_parameters!( LineShaderParams {
    u_model_view_proj@ model_view_proj: [[f32; 4]; 4],
});
