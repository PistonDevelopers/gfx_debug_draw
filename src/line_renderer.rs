use gfx::batch::bind;

use gfx::{
    BufferHandle,
    BufferUsage,
    Device,
    DeviceExt,
    DrawState,
    Frame,
    Graphics,
    Mesh,
    PrimitiveType,
    ProgramError,
    ProgramHandle,
    Resources,
    ShaderSource,
    ToSlice,
    VertexCount,
};

use utils::{grow_buffer, MAT4_ID};
use std::marker::PhantomData;

pub struct LineRenderer<D: Device> {
    program: ProgramHandle<D::Resources>,
    state: DrawState,
    vertex_data: Vec<Vertex>,
    vertex_buffer: BufferHandle<D::Resources, Vertex>,
    params: LineShaderParams<D::Resources>,
}

impl<D: Device> LineRenderer<D> {

    pub fn new(graphics: &mut Graphics<D>, initial_buffer_size: usize) -> Result<LineRenderer<D>, ProgramError> {

        let shader_model = graphics.device.get_capabilities().shader_model;

        let vertex = ShaderSource {
            glsl_120: Some(VERTEX_SRC[0]),
            glsl_150: Some(VERTEX_SRC[0]),
            .. ShaderSource::empty()
        };

        let fragment = ShaderSource {
            glsl_120: Some(FRAGMENT_SRC[0]),
            glsl_150: Some(FRAGMENT_SRC[0]),
            .. ShaderSource::empty()
        };

        let program = match graphics.device.link_program(
            vertex.choose(shader_model).unwrap(),
            fragment.choose(shader_model).unwrap()
        ) {
            Ok(program_handle) => program_handle,
            Err(e) => return Err(e),
        };

        let vertex_buffer = graphics.device.create_buffer::<Vertex>(initial_buffer_size, BufferUsage::Dynamic);

        Ok(LineRenderer {
            vertex_data: Vec::new(),
            program: program,
            state: DrawState::new(),
            vertex_buffer: vertex_buffer,
            params: LineShaderParams {
                u_model_view_proj: MAT4_ID,
                _marker: PhantomData,
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
    pub fn render(
        &mut self,
        graphics: &mut Graphics<D>,
        frame: &Frame<D::Resources>,
        projection: [[f32; 4]; 4],
    ) {
        self.params.u_model_view_proj = projection;

        if self.vertex_data.len() > self.vertex_buffer.len() {
            self.vertex_buffer = grow_buffer(graphics, self.vertex_buffer.clone(), self.vertex_data.len());
        }

        graphics.device.update_buffer(self.vertex_buffer.clone(), &self.vertex_data[..], 0);

        let mesh = Mesh::from_format(
            self.vertex_buffer.clone(),
            self.vertex_data.len() as VertexCount
        );

        let slice = mesh.to_slice(PrimitiveType::Line);

        graphics.renderer.draw(
            &bind(&self.state, &mesh, slice, &self.program, &self.params),
            &frame
        ).unwrap();

        self.vertex_data.clear();
    }
}

static VERTEX_SRC: [&'static [u8]; 2] = [
b"
    #version 120

    uniform mat4 u_model_view_proj;
    attribute vec3 position;
    attribute vec4 color;
    varying vec4 v_color;

    void main() {
        gl_Position = u_model_view_proj * vec4(position, 1.0);
        v_color = color;
    }
",
b"
    #version 150 core

    uniform mat4 u_model_view_proj;
    in vec3 position;
    in vec4 color;
    out vec4 v_color;

    void main() {
        gl_Position = u_model_view_proj * vec4(position, 1.0);
        v_color = color;
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

#[vertex_format]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

#[shader_param]
struct LineShaderParams<R: Resources> {
    u_model_view_proj: [[f32; 4]; 4],
    _marker: PhantomData<R>,
}
