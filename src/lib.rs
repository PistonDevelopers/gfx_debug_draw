#![feature(plugin)]
#![plugin(gfx_macros)]

extern crate gfx;

use gfx::batch::{
    BatchError,
    RefBatch,
};

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
    VertexCount,
};

static MAT4_ID: [[f32; 4]; 4] =
[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

static VERTEX_SRC: &'static [u8] = b"
    #version 150 core

    uniform mat4 u_model_view_proj;
    in vec3 position;
    in vec4 color;
    out vec4 v_color;

    void main() {
        gl_Position = u_model_view_proj * vec4(position, 1.0);
        v_color = color;
    }
";

static FRAGMENT_SRC: &'static [u8] = b"
    #version 150

    in vec4 v_color;
    out vec4 out_color;

    void main() {
        out_color = v_color;
    }
";

#[vertex_format]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

#[shader_param]
struct Params {
    u_model_view_proj: [[f32; 4]; 4],
}

pub struct DebugRenderer {
    program: ProgramHandle<GlResources>,
    state: DrawState,
    line_vertex_data: Vec<Vertex>,
    line_index_data: Vec<u32>,
    vertex_buffer: BufferHandle<GlResources, Vertex>,
    vertex_buffer_size: usize,
    params: Params,
}

impl DebugRenderer {

    pub fn new(graphics: &mut Graphics<GlDevice>, initial_buffer_size: usize) -> Result<DebugRenderer, ProgramError> {

        let program = match graphics.device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone()) {
            Ok(program_handle) => program_handle,
            Err(e) => return Err(e),
        };

        let vertex_buffer = graphics.device.create_buffer::<Vertex>(initial_buffer_size, BufferUsage::Dynamic);

        Ok(DebugRenderer {
            line_vertex_data: Vec::new(),
            line_index_data: Vec::new(),
            program: program,
            state: DrawState::new(),
            vertex_buffer: vertex_buffer,
            vertex_buffer_size: initial_buffer_size,
            params: Params { u_model_view_proj: MAT4_ID },
        })
    }

    ///
    /// Add a line to the batch to be drawn on 'render'
    ///
    pub fn draw_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        let index = self.line_vertex_data.len() as u32;
        self.line_vertex_data.push(Vertex{position: start, color: color});
        self.line_vertex_data.push(Vertex{position: end, color: color});
        self.line_index_data.push(index);
        self.line_index_data.push(index + 1);
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
        self.params.u_model_view_proj = projection;

        self.grow_vertex_buffer(graphics);
        graphics.device.update_buffer(self.vertex_buffer.clone(), &self.line_vertex_data[..], 0);

        match self.make_batch(graphics) {
            Ok(batch) =>  {
                graphics.draw(&batch, &self.params, frame);
            },
            Err(e) => {
                println!("Error creating debug render batch: {:?}", e);
            },
        }

        self.line_vertex_data.clear();
        self.line_index_data.clear();
    }

    ///
    /// Grow the vertex buffer if necessary
    ///
    fn grow_vertex_buffer(&mut self, graphics: &mut Graphics<GlDevice>) {

        let required_size = self.line_vertex_data.len();

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
    fn make_batch(&mut self, graphics: &mut Graphics<GlDevice>) -> Result<RefBatch<Params>, BatchError> {
        let mesh = Mesh::from_format(
            self.vertex_buffer.clone(),
            self.line_vertex_data.len() as VertexCount
        );
        let slice = mesh.to_slice(PrimitiveType::Line);
        graphics.make_batch(&self.program, &mesh, slice, &self.state)
    }
}
