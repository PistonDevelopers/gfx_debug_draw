#![feature(plugin)]
#![plugin(gfx_macros)]

extern crate gfx;

use gfx::batch::RefBatch;

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
    ToSlice,
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
    batch: Option<RefBatch<Params>>,
    params: Params,
}

impl DebugRenderer {

    pub fn new(device: &mut GlDevice) -> Result<DebugRenderer, ProgramError> {

        let program = match device.link_program(VERTEX_SRC.clone(), FRAGMENT_SRC.clone()) {
            Ok(program_handle) => program_handle,
            Err(e) => return Err(e),
        };

        Ok(DebugRenderer {
            line_vertex_data: Vec::new(),
            line_index_data: Vec::new(),
            program: program,
            state: DrawState::new(),
            batch: None,
            params: Params { u_model_view_proj: MAT4_ID },
        })
    }

    pub fn add_line(&mut self, start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
        let index = self.line_vertex_data.len() as u32;

        self.line_vertex_data.push(Vertex{position: start, color: color});
        self.line_vertex_data.push(Vertex{position: end, color: color});

        self.line_index_data.push(index);
        self.line_index_data.push(index + 1);

        // Invalidate batch
        self.batch = None;
    }

    pub fn clear(&mut self) {
        self.line_vertex_data.clear();
        self.line_index_data.clear();
    }

    pub fn render(
        &mut self,
        graphics: &mut Graphics<GlDevice>,
        frame: &Frame<GlResources>,
        projection: [[f32; 4]; 4],
    ) {
        self.params.u_model_view_proj = projection;

        if self.batch == None {
            self.make_batch(graphics);
        }

        if let Some(ref batch) = self.batch {
            match graphics.draw(&batch, &self.params, frame) {
                Err(_) => println!("Error on draw."),
                _ => (),
            }
        }
    }

    fn make_batch(&mut self, graphics: &mut Graphics<GlDevice>) {

        let vertex_buffer_size = self.line_vertex_data.len();
        let vertex_buffer = graphics.device.create_buffer::<Vertex>(vertex_buffer_size, BufferUsage::Dynamic);
        graphics.device.update_buffer(vertex_buffer, &self.line_vertex_data[..], 0);

        let mesh = Mesh::from_format(vertex_buffer, vertex_buffer_size as u32);

        // TODO VertexSlice(PrimitiveType::Line, 0, vertex_count - 1)
        let slice = graphics.device
            .create_buffer_static::<u32>(&self.line_index_data[..])
            .to_slice(PrimitiveType::Line);

        let batch = graphics.make_batch(&self.program, &mesh, slice, &self.state).unwrap();

        self.batch = Some(batch);
    }
}
