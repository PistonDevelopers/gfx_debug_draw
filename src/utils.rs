use gfx::{
    buffer,
    handle,
    memory,
    Bind,
    Factory,
    Resources,
};

pub fn grow_buffer<R: Resources, F: Factory<R>, T>(
    factory: &mut F,
    buffer: &handle::Buffer<R, T>,
    buffer_role: buffer::Role,
    required_size: usize,
) -> handle::Buffer<R, T> {
    let mut size = buffer.len();
    while size < required_size {
        size *= 2;
    }
    factory.create_buffer(size, buffer_role, memory::Usage::Dynamic, Bind::empty())
        .expect("Could not create buffer")
}
