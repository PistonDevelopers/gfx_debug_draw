use std::collections::VecDeque;
use std::mem;
use std::sync::{Arc,Mutex,Once,ONCE_INIT};
use gfx;
use gfx::traits::*;
use gfx_text;
use vecmath::*;

use line_renderer::LineRenderer;

type WorldPosition = [f32; 3];
type ScreenPosition = [i32; 2];
type Color = [f32; 4];
type Size = f32;

enum DrawCommand {
    DrawLine(WorldPosition, WorldPosition, Color),
    DrawMarker(WorldPosition, Size, Color),
    DrawScreenText(String, ScreenPosition, Color),
    DrawWorldText(String, WorldPosition, Color),
}

#[derive(Clone)]
struct CommandQueue {
    queue: Arc<Mutex<VecDeque<DrawCommand>>>
}

static mut command_queue_singleton: *const CommandQueue = 0 as *const CommandQueue;

impl CommandQueue {

    fn push_back(&mut self, command: DrawCommand) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(command);
    }

    fn pop_front(&mut self) -> Option<DrawCommand> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }

    fn instance() -> CommandQueue {
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                let command_queue = CommandQueue {
                    queue: Arc::new(Mutex::new(VecDeque::<DrawCommand>::new()))
                };
                command_queue_singleton = mem::transmute(Box::new(command_queue));
            });
            (*command_queue_singleton).clone()
        }
    }
}

pub fn draw_marker(position: [f32; 3], size: f32, color: [f32; 4]) {
    CommandQueue::instance().push_back(DrawCommand::DrawMarker(position, size, color));
}

pub fn draw_line(start: [f32; 3], end: [f32; 3], color: [f32; 4]) {
    CommandQueue::instance().push_back(DrawCommand::DrawLine(start, end, color));
}

pub fn draw_text_on_screen(text: &str, screen_position: [i32; 2], color: [f32; 4]) {
    CommandQueue::instance().push_back(DrawCommand::DrawScreenText(text.to_string(), screen_position, color));
}

pub fn draw_text_at_position(text: &str, position: [f32; 3], color: [f32; 4]) {
    CommandQueue::instance().push_back(DrawCommand::DrawWorldText(text.to_string(), position, color));
}

#[derive(Debug)]
pub enum DebugRendererError {
    ShaderProgramError(gfx::ProgramError),
    BufferUpdateError(gfx::device::BufferUpdateError),
    GfxTextError(gfx_text::Error)
}

impl From<gfx::ProgramError> for DebugRendererError {
    fn from(err: gfx::ProgramError) -> DebugRendererError {
        DebugRendererError::ShaderProgramError(err)
    }
}

impl From<gfx::device::BufferUpdateError> for DebugRendererError {
    fn from(err: gfx::device::BufferUpdateError) -> DebugRendererError {
        DebugRendererError::BufferUpdateError(err)
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

impl<R: gfx::Resources, F: Factory<R>> Drop for DebugRenderer<R, F> {

    /// Frees memory alocated for the command queue singleton
    ///
    /// Could be leaky if a `gfx_debug_draw::draw_*` method is called without
    /// ever instantiating a DebugRenderer. Also could be problematic if there
    /// are multiple DebugRenderer instances.
    ///
    /// Should look into freeing memory on `rt::at_exit` if/when that becomes stable
    /// See http://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
    fn drop(&mut self) {
        unsafe {
            if command_queue_singleton != 0 as *const _ {
                let command_queue: Box<CommandQueue> = mem::transmute(command_queue_singleton);
                drop(command_queue);
                command_queue_singleton = 0 as *const _;
            }
        }
    }
}

impl<R: gfx::Resources, F: Factory<R>> DebugRenderer<R, F> {

    pub fn new (
        factory: F,
        text_renderer: gfx_text::Renderer<R, F>,
        initial_buffer_size: usize,
    ) -> Result<DebugRenderer<R, F>, DebugRendererError> {

        let mut factory = factory;
        let line_renderer = try!(LineRenderer::new(&mut factory, initial_buffer_size));

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

    pub fn render<S: gfx::Stream<R>> (
        &mut self,
        stream: &mut S,
        projection: [[f32; 4]; 4],
    ) -> Result<(), DebugRendererError> {

        loop {
            match CommandQueue::instance().pop_front() {
                Some(DrawCommand::DrawLine(start, end, color)) => {
                    self.draw_line(start, end, color);
                },
                Some(DrawCommand::DrawMarker(position, size, color)) => {
                    self.draw_marker(position, size, color);
                },
                Some(DrawCommand::DrawScreenText(ref text, position, color)) => {
                    self.draw_text_on_screen(&text, position, color);
                },
                Some(DrawCommand::DrawWorldText(ref text, position, color)) => {
                    self.draw_text_at_position(&text, position, color);
                }
                None => break
            }
        }

        try!(self.line_renderer.render(stream, &mut self.factory, projection));
        try!(self.text_renderer.draw_at(stream, projection));
        Ok(())
    }
}
